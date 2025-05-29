use crate::domain::entities::holiday::Holiday;
use crate::domain::services::holiday_service::HolidayScrapingService;
use crate::shared::errors::app_error::{AppError, AppResult};
use async_trait::async_trait;
use chrono::NaiveDate;
use scraper::{Html, Selector};
use std::collections::HashMap;

// WebScrapingService struct is responsible for scraping holiday data from a webpage.
pub struct WebScrapingService;

impl WebScrapingService {
    // Constructor to initialize WebScrapingService
    pub fn new() -> Self {
        Self
    }

    // Returns a HashMap mapping month names in Indonesian to their respective month codes (e.g., "januari" -> "01").
    fn get_month_map() -> HashMap<&'static str, &'static str> {
        HashMap::from([
            ("januari", "01"),
            ("februari", "02"),
            ("maret", "03"),
            ("april", "04"),
            ("mei", "05"),
            ("juni", "06"),
            ("juli", "07"),
            ("agustus", "08"),
            ("september", "09"),
            ("oktober", "10"),
            ("november", "11"),
            ("desember", "12"),
        ])
    }

    // Parses a date given the year, month code (e.g., "01" for January), and the day as a string.
    // Returns an `AppResult<NaiveDate>` where errors will be mapped to `AppError`.
    fn parse_date(&self, year: i32, month_code: &str, day: &str) -> AppResult<NaiveDate> {
        let date_string = format!("{}-{}-{:0>2}", year, month_code, day.trim());
        NaiveDate::parse_from_str(&date_string, "%Y-%m-%d")
            .map_err(|e| AppError::BadRequest(format!("Invalid date format: {}", e)))
    }
}

#[async_trait]
impl HolidayScrapingService for WebScrapingService {
    // Asynchronously scrapes holiday data for a specific year from an external website.
    // Returns a list of holidays (`Vec<Holiday>`) or an error if the scraping fails.
    async fn scrape_holidays_for_year(&self, year: i32) -> AppResult<Vec<Holiday>> {
        let url = format!("https://www.tanggalan.com/{}", year); // URL to scrape

        println!("Scraping holidays from: {}", url);

        // Make the HTTP request to the URL
        let response = reqwest::get(&url).await?;
        let html_content = response.text().await?;
        let document = Html::parse_document(&html_content);

        // Define a CSS selector to select the list of holidays for each month
        let ul_selector = Selector::parse("article ul")
            .map_err(|e| AppError::InternalServer(format!("CSS selector error: {:?}", e)))?;

        let month_map = Self::get_month_map(); // Get month-to-code mapping
        let mut holidays = Vec::new(); // Store the holidays

        // Iterate through each list (month section) in the document
        for ul in document.select(&ul_selector) {
            // Extract month name (from the first link within each list)
            let month_str = ul
                .select(&Selector::parse("li a").unwrap())
                .next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();

            let month = month_str
                .to_lowercase()
                .replace(char::is_numeric, "")
                .trim()
                .to_string();

            // Get the month code (e.g., "01" for January)
            let month_code = month_map.get(month.as_str()).unwrap_or(&"01");

            // Define a CSS selector to select the rows containing holiday entries
            let tr_selector = Selector::parse("li:nth-child(4) tbody tr")
                .map_err(|e| AppError::InternalServer(format!("CSS selector error: {:?}", e)))?;

            // Iterate over each holiday entry
            for tr in ul.select(&tr_selector) {
                let day = tr
                    .select(&Selector::parse("td:first-child").unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                let description = tr
                    .select(&Selector::parse("td:nth-child(2)").unwrap())
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();

                if !day.trim().is_empty() && !description.trim().is_empty() {
                    // Parse the date
                    match self.parse_date(year, month_code, &day) {
                        Ok(date) => {
                            // Check if the description indicates a joint leave
                            let is_joint_leave = description.contains("Cuti Bersama");
                            // Create a Holiday entity and add it to the list
                            let holiday =
                                Holiday::new(date, description.trim().to_string(), is_joint_leave);
                            holidays.push(holiday);
                        }
                        Err(e) => {
                            eprintln!(
                                "Failed to parse date for {}-{}-{}: {}",
                                year, month_code, day, e
                            );
                            continue; // Skip this holiday entry if date parsing fails
                        }
                    }
                }
            }
        }

        // If no holidays were found, return an error
        if holidays.is_empty() {
            return Err(AppError::ExternalService(format!(
                "No holidays found for year {} from external source",
                year
            )));
        }

        println!(
            "Successfully scraped {} holidays for year {}",
            holidays.len(),
            year
        );
        Ok(holidays)
    }
}
