use crate::application::dto::api_response::ApiResponse;
use crate::application::dto::holiday_dto::HolidayDto;
use crate::application::use_cases::{
    get_holidays::GetHolidaysUseCase, scrape_holidays::ScrapeHolidaysUseCase,
};
use axum::{extract::Path, response::Json, Extension};
use std::collections::HashMap;
use std::sync::Arc;
pub async fn root() -> &'static str {
    "
🎉 Welcome to the Holiday API! 

Available endpoints:

- GET /scrape/{year} 
    - Scrape holidays for the specific year from an external source.
    - Example: GET /scrape/2023

- GET /libur/{year}
    - Get holidays for a specific year as a list of holidays.
    - Example: GET /libur/2023
    - Returns a list of holidays for the given year.

- GET /libur-grouped/{year}
    - Get holidays for a specific year, grouped by type (e.g., joint leave vs. non-joint leave).
    - Example: GET /libur-grouped/2023
    - Returns holidays grouped by their type.
    "
}

pub async fn scrape_holidays(
    Path(year): Path<i32>,
    Extension(use_case): Extension<Arc<ScrapeHolidaysUseCase>>,
) -> Result<Json<ApiResponse<Vec<HolidayDto>>>, Json<ApiResponse<String>>> {
    match use_case.execute(year).await {
        Ok(holidays) => {
            let response = ApiResponse::success(holidays, "Holidays scraped successfully");
            Ok(Json(response))
        }
        Err(e) => {
            let response = ApiResponse::error(500, &e.to_string(), String::new());
            Err(Json(response))
        }
    }
}

pub async fn get_holidays(
    Path(year): Path<i32>,
    Extension(use_case): Extension<Arc<GetHolidaysUseCase>>,
) -> Result<Json<ApiResponse<Vec<HolidayDto>>>, Json<ApiResponse<String>>> {
    match use_case.execute(year).await {
        Ok(holidays) => {
            let response = ApiResponse::success(holidays, "Holidays retrieved successfully");
            Ok(Json(response))
        }
        Err(e) => {
            let response = ApiResponse::error(404, &e.to_string(), String::new());
            Err(Json(response))
        }
    }
}

pub async fn get_holidays_grouped(
    Path(year): Path<i32>,
    Extension(use_case): Extension<Arc<GetHolidaysUseCase>>,
) -> Result<Json<ApiResponse<HashMap<String, Vec<HolidayDto>>>>, Json<ApiResponse<String>>> {
    match use_case.execute_grouped(year).await {
        Ok(grouped_holidays) => {
            let response = ApiResponse::success(
                grouped_holidays.to_hashmap(),
                "Holidays retrieved successfully",
            );
            Ok(Json(response))
        }
        Err(e) => {
            let response = ApiResponse::error(404, &e.to_string(), String::new());
            Err(Json(response))
        }
    }
}
