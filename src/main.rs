use calendar_indonesia::{
    application::use_cases::{
        get_holidays::GetHolidaysUseCase, scrape_holidays::ScrapeHolidaysUseCase,
    },
    config::Config,
    infrastructure::{
        external::scraper_service::WebScrapingService,
        persistence::file_repository::FileHolidayRepository,
        scheduler::holiday_scheduler::start_periodic_scraper,
        web::routes::holiday_routes::create_routes,
    },
};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment variables (e.g., host and port)
    let config = Config::from_env();

    // Print startup information to the console
    println!("Starting Holiday API server...");
    println!("Server will run at http://{}:{}", config.host, config.port);

    // Setup the repository for storing holiday data (uses File-based storage)
    let holiday_repository = Arc::new(FileHolidayRepository::new(config.data_dir.clone()));

    // Setup the web scraping service to gather holidays from an external source
    let scraping_service = Arc::new(WebScrapingService::new());

    // Setup use cases to interact with the repository and scraping service
    let get_holidays_use_case = Arc::new(GetHolidaysUseCase::new(holiday_repository.clone()));
    let scrape_holidays_use_case = Arc::new(ScrapeHolidaysUseCase::new(
        holiday_repository.clone(),
        scraping_service,
    ));

    // Setup the Axum routes
    let app = create_routes(get_holidays_use_case, scrape_holidays_use_case.clone());

    // Start a periodic scraper that will scrape holidays at scheduled intervals
    start_periodic_scraper(scrape_holidays_use_case.clone()).await;

    // Start the server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;

    println!("🚀 Server started successfully at http://{}", addr);
    println!("📚 Available endpoints:");
    println!("   GET  /                     - Welcome message");
    println!("   GET  /scrape/{{year}}        - Scrape holidays for a specific year");
    println!("   GET  /libur/{{year}}         - Get holidays for a specific year");
    println!("   GET  /libur/{{year}}/grouped - Get holidays for a specific year, grouped by type");

    // Serve the app
    axum::serve(listener, app).await?;

    Ok(())
}
