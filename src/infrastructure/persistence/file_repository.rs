use crate::domain::entities::holiday::{Holiday, HolidayRaw};
use crate::domain::repositories::holiday_repository::HolidayRepository;
use crate::shared::errors::app_error::{AppError, AppResult};
use async_trait::async_trait;
use chrono::NaiveDate;
use std::fs;
use std::path::Path;

// FileHolidayRepository struct stores the directory path where holiday data files are located.
pub struct FileHolidayRepository {
    data_dir: String,
}

impl FileHolidayRepository {
    // Constructor to initialize FileHolidayRepository with a specific data directory.
    pub fn new(data_dir: String) -> Self {
        Self { data_dir }
    }

    // Returns the file path for the given year, constructing it using the data directory and year.
    fn get_file_path(&self, year: i32) -> String {
        format!("{}/{}.json", self.data_dir, year)
    }

    // Ensures the data directory exists by creating it if necessary.
    fn ensure_data_dir_exists(&self) -> AppResult<()> {
        fs::create_dir_all(&self.data_dir)?;
        Ok(())
    }

    // Converts a raw holiday record to a domain-specific holiday entity.
    // It parses the date from string format and checks if it contains "Cuti Bersama".
    fn convert_raw_to_domain(&self, raw: HolidayRaw) -> AppResult<Holiday> {
        let date = NaiveDate::parse_from_str(&raw.tanggal, "%Y-%m-%d")
            .map_err(|e| AppError::BadRequest(format!("Invalid date format: {}", e)))?;

        let is_joint_leave = raw.keterangan.contains("Cuti Bersama");

        Ok(Holiday::new(date, raw.keterangan, is_joint_leave))
    }

    // Converts a domain-specific holiday entity back to a raw data format.
    fn convert_domain_to_raw(&self, holiday: Holiday) -> HolidayRaw {
        HolidayRaw {
            tanggal: holiday.format_date(),
            keterangan: holiday.description,
        }
    }
}

#[async_trait]
impl HolidayRepository for FileHolidayRepository {
    // Asynchronously saves a list of holidays to a file corresponding to the given year.
    // It ensures the data directory exists, converts holidays to raw format, and writes to JSON.
    async fn save_holidays(&self, holidays: Vec<Holiday>, year: i32) -> AppResult<()> {
        self.ensure_data_dir_exists()?;

        // Convert each holiday into its raw data format
        let raw_holidays: Vec<HolidayRaw> = holidays
            .into_iter()
            .map(|h| self.convert_domain_to_raw(h))
            .collect();

        // Prepare the file path and serialize the holidays into pretty JSON format
        let file_path = self.get_file_path(year);
        let json_data = serde_json::to_string_pretty(&raw_holidays)?;

        // Write the JSON data to a file
        fs::write(&file_path, json_data)?;
        println!("File {} successfully created", file_path);

        Ok(())
    }

    // Asynchronously retrieves holidays from a file based on the given year.
    // If the file does not exist, it returns a NotFound error.
    async fn get_holidays_by_year(&self, year: i32) -> AppResult<Vec<Holiday>> {
        let file_path = self.get_file_path(year);

        if !Path::new(&file_path).exists() {
            return Err(AppError::NotFound(format!(
                "Holiday data not found for year {}",
                year
            )));
        }

        let contents = fs::read_to_string(&file_path)?;
        let raw_holidays: Vec<HolidayRaw> = serde_json::from_str(&contents)?;

        // Convert raw holidays into domain-specific holiday entities
        let holidays: Result<Vec<Holiday>, AppError> = raw_holidays
            .into_iter()
            .map(|raw| self.convert_raw_to_domain(raw))
            .collect();

        holidays
    }

    // Asynchronously checks if holiday data exists for the given year by checking if the file exists.
    async fn holidays_exist_for_year(&self, year: i32) -> AppResult<bool> {
        let file_path = self.get_file_path(year);
        Ok(Path::new(&file_path).exists())
    }
}
