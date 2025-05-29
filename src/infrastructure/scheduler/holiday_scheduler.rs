use chrono::{Datelike, Duration as ChronoDuration, Local, TimeZone};
use std::sync::Arc;
use tokio::time::sleep;

use crate::application::use_cases::scrape_holidays::ScrapeHolidaysUseCase;

pub async fn start_periodic_scraper(use_case: Arc<ScrapeHolidaysUseCase>) {
    tokio::spawn(async move {
        println!("🕐 Periodic scraper started - will check daily at 00:01");

        loop {
            let now = Local::now();

            // Calculate next 00:01
            let tomorrow = now.date_naive() + ChronoDuration::days(1);
            let next_check = Local
                .from_local_datetime(&tomorrow.and_hms_opt(0, 1, 0).unwrap())
                .unwrap();

            let sleep_duration = (next_check - now).to_std().unwrap();

            println!(
                "⏰ Next check at: {}",
                next_check.format("%Y-%m-%d %H:%M:%S")
            );
            sleep(sleep_duration).await;

            // Check current date after waking up
            let now = Local::now();

            // Check if the current date is January 1st
            if now.month() == 1 && now.day() == 1 {
                let year = now.year();
                println!(
                    "📅 January 1st detected! Starting automatic scrape for year {}",
                    year
                );

                match use_case.execute(year).await {
                    Ok(holidays) => {
                        println!("🎉 Automatic scraping completed successfully for year {} - scraped {} holidays", year, holidays.len());
                    }
                    Err(e) => {
                        println!("❌ Automatic scraping failed for year {}: {:?}", year, e);
                    }
                }
            } else {
                println!(
                    "📆 Current date: {}-{:02}-{:02} (not January 1st)",
                    now.year(),
                    now.month(),
                    now.day()
                );
            }
        }
    });
}
