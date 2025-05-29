use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq)]
pub struct Holiday {
    pub date: NaiveDate,
    pub description: String,
    pub is_joint_leave: bool,
}

impl Holiday {
    // Constructor to create a new Holiday instance.
    pub fn new(date: NaiveDate, description: String, is_joint_leave: bool) -> Self {
        Self {
            date,
            description,
            is_joint_leave,
        }
    }

    // Getter method to check if the holiday is a joint leave.
    pub fn is_joint_leave(&self) -> bool {
        self.is_joint_leave
    }

    // Method to format the date as a string in the "YYYY-MM-DD" format.
    pub fn format_date(&self) -> String {
        self.date.format("%Y-%m-%d").to_string()
    }
}

// Struct for raw data representation, used for persisting holiday data from scraping or other sources.
#[derive(Serialize, Deserialize, Clone)]
pub struct HolidayRaw {
    pub tanggal: String,
    pub keterangan: String,
}
