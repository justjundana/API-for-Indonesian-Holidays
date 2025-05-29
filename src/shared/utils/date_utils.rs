use chrono::{Datelike, Local, NaiveDate};

// Retrieves the current year based on the local system time.
pub fn get_current_year() -> i32 {
    Local::now().year()
}

// Validates if the given year is between 1900 and 2100.
pub fn is_valid_year(year: i32) -> bool {
    year >= 1900 && year <= 2100
}

// Formats a date into the Indonesian format, e.g., "15 Agustus 2023".
pub fn format_date_indonesia(date: &NaiveDate) -> String {
    // Array of month names in Indonesian
    let months = [
        "Januari",
        "Februari",
        "Maret",
        "April",
        "Mei",
        "Juni",
        "Juli",
        "Agustus",
        "September",
        "Oktober",
        "November",
        "Desember",
    ];

    // Get the month name from the array using the month number (adjusted for 0-based index)
    let month_name = months
        .get((date.month() as usize).saturating_sub(1)) // Subtract 1 because array indexing starts at 0
        .unwrap_or(&"Invalid"); // If month is invalid, return "Invalid"

    // Return the formatted date as "day month year"
    format!("{} {} {}", date.day(), month_name, date.year())
}
