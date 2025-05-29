use crate::domain::entities::holiday::Holiday;
use crate::shared::errors::app_error::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait HolidayScrapingService: Send + Sync {
    // Async method to scrape holidays for a given year.
    // It returns a vector of `Holiday` objects for the specified year, or an error (`AppResult`).
    async fn scrape_holidays_for_year(&self, year: i32) -> AppResult<Vec<Holiday>>;
}

pub struct HolidayDomainService;

impl HolidayDomainService {
    // Constructor for `HolidayDomainService`. It doesn't require any parameters to create.
    pub fn new() -> Self {
        Self
    }

    // Method to group holidays into two categories: joint leave and non-joint leave.
    // It takes a vector of `Holiday` and returns a tuple containing two vectors: one for joint leaves and another for non-joint leaves.
    pub fn group_holidays_by_type(&self, holidays: Vec<Holiday>) -> (Vec<Holiday>, Vec<Holiday>) {
        let mut joint_leave = Vec::new();
        let mut non_joint_leave = Vec::new();

        for holiday in holidays {
            // Classify holidays based on the `is_joint_leave` flag.
            if holiday.is_joint_leave() {
                joint_leave.push(holiday);
            } else {
                non_joint_leave.push(holiday);
            }
        }

        (joint_leave, non_joint_leave)
    }
}
