use crate::application::dto::holiday_dto::HolidayDto;
use crate::domain::repositories::holiday_repository::HolidayRepository;
use crate::domain::services::holiday_service::HolidayScrapingService;
use crate::shared::errors::app_error::AppResult;
use std::sync::Arc;

// The ScrapeHolidaysUseCase struct orchestrates the process of scraping holidays and saving them to the repository.
pub struct ScrapeHolidaysUseCase {
    holiday_repository: Arc<dyn HolidayRepository>,
    scraping_service: Arc<dyn HolidayScrapingService>,
}

impl ScrapeHolidaysUseCase {
    // Constructor to initialize ScrapeHolidaysUseCase with dependencies: repository and scraping service.
    pub fn new(
        holiday_repository: Arc<dyn HolidayRepository>,
        scraping_service: Arc<dyn HolidayScrapingService>,
    ) -> Self {
        Self {
            holiday_repository,
            scraping_service,
        }
    }

    // The main method to execute the use case: scrape holidays and save them to the repository.
    // It also converts the results into DTOs before returning them.
    pub async fn execute(&self, year: i32) -> AppResult<Vec<HolidayDto>> {
        println!("Starting to scrape holidays for year {}", year);

        // Step 1: Scrape holidays using the scraping service.
        let holidays = self.scraping_service.scrape_holidays_for_year(year).await?;

        // Step 2: Save the scraped holidays to the holiday repository.
        self.holiday_repository
            .save_holidays(holidays.clone(), year) // Pass a clone of the holidays since it's being used again
            .await?;

        println!(
            "Successfully scraped and saved {} holidays for year {}",
            holidays.len(),
            year
        );

        // Step 3: Convert the domain holidays into DTOs and return them.
        let holiday_dtos: Vec<HolidayDto> = holidays.into_iter().map(HolidayDto::from).collect();

        Ok(holiday_dtos)
    }
}
