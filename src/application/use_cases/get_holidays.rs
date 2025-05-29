use crate::application::dto::holiday_dto::{GroupedHolidaysDto, HolidayDto};
use crate::domain::repositories::holiday_repository::HolidayRepository;
use crate::domain::services::holiday_service::HolidayDomainService;
use crate::shared::errors::app_error::{AppError, AppResult};
use std::sync::Arc;

// GetHolidaysUseCase struct is responsible for retrieving holidays from the repository
// and returning them as DTOs, either as a simple list or grouped by type (e.g., joint leave vs. non-joint leave).
pub struct GetHolidaysUseCase {
    holiday_repository: Arc<dyn HolidayRepository>,
    holiday_service: HolidayDomainService,
}

impl GetHolidaysUseCase {
    // Constructor to initialize GetHolidaysUseCase with the holiday repository.
    // The holiday service is initialized with a default constructor.
    pub fn new(holiday_repository: Arc<dyn HolidayRepository>) -> Self {
        Self {
            holiday_repository,
            holiday_service: HolidayDomainService::new(),
        }
    }

    // Method to fetch holidays for a given year and return them as a simple list of HolidayDto.
    // If no holidays are found, it returns a NotFound error.
    pub async fn execute(&self, year: i32) -> AppResult<Vec<HolidayDto>> {
        // Fetch holidays from the repository
        let holidays = self.holiday_repository.get_holidays_by_year(year).await?;

        // If no holidays are found, return a NotFound error
        if holidays.is_empty() {
            return Err(AppError::NotFound(format!(
                "No holidays found for year {}",
                year
            )));
        }

        // Convert each Holiday to a HolidayDto
        let holiday_dtos: Vec<HolidayDto> = holidays.into_iter().map(HolidayDto::from).collect();

        Ok(holiday_dtos)
    }

    // Method to fetch holidays for a given year and return them grouped by holiday type (e.g., joint leave vs. non-joint leave).
    // If no holidays are found, it returns a NotFound error.
    pub async fn execute_grouped(&self, year: i32) -> AppResult<GroupedHolidaysDto> {
        // Fetch holidays from the repository
        let holidays = self.holiday_repository.get_holidays_by_year(year).await?;

        // If no holidays are found, return a NotFound error
        if holidays.is_empty() {
            return Err(AppError::NotFound(format!(
                "No holidays found for year {}",
                year
            )));
        }

        // Use the holiday service to group holidays by type (joint leave vs non-joint leave)
        let (joint_leave, non_joint_leave) = self.holiday_service.group_holidays_by_type(holidays);

        Ok(GroupedHolidaysDto::new(joint_leave, non_joint_leave))
    }
}
