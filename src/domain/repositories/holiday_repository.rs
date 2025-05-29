use crate::domain::entities::holiday::Holiday;
use crate::shared::errors::app_error::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait HolidayRepository: Send + Sync {
    // Async method to save holidays for a given year.
    // It accepts a vector of `Holiday` objects and the year, returning a `Result` with `()` for success or an error.
    async fn save_holidays(&self, holidays: Vec<Holiday>, year: i32) -> AppResult<()>;

    // Async method to retrieve holidays for a specific year.
    // Returns a `Result` containing a vector of `Holiday` objects for the given year.
    async fn get_holidays_by_year(&self, year: i32) -> AppResult<Vec<Holiday>>;

    // Async method to check if holidays exist for a given year.
    // Returns a `Result` with a boolean value (`true` if holidays exist for the year, otherwise `false`).
    async fn holidays_exist_for_year(&self, year: i32) -> AppResult<bool>;
}
