use crate::application::use_cases::{
    get_holidays::GetHolidaysUseCase, scrape_holidays::ScrapeHolidaysUseCase,
};
use crate::infrastructure::web::handlers::holiday_handler::{
    get_holidays, get_holidays_grouped, root, scrape_holidays,
};
use axum::{routing::get, Extension, Router};
use std::sync::Arc;

pub fn create_routes(
    get_holidays_use_case: Arc<GetHolidaysUseCase>,
    scrape_holidays_use_case: Arc<ScrapeHolidaysUseCase>,
) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/scrape/{year}", get(scrape_holidays))
        .route("/libur/{year}", get(get_holidays))
        .route("/libur/{year}/grouped", get(get_holidays_grouped))
        .layer(Extension(get_holidays_use_case))
        .layer(Extension(scrape_holidays_use_case))
}
