use crate::domain::entities::holiday::Holiday;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// HolidayDto is a Data Transfer Object (DTO) that is used to transfer holiday data over the API.
// It includes:
// - date: The holiday date as a string, formatted in a specific way.
// - description: The description or name of the holiday.
// - is_joint_leave: A boolean flag indicating whether the holiday is a joint leave (e.g., Cuti Bersama).
#[derive(Serialize, Deserialize)]
pub struct HolidayDto {
    pub date: String,
    pub description: String,
    pub is_joint_leave: bool,
}

impl From<Holiday> for HolidayDto {
    // The From trait is implemented to convert a Holiday entity to a HolidayDto.
    // This is useful when you need to convert the domain model to a DTO for API responses.
    fn from(holiday: Holiday) -> Self {
        Self {
            date: holiday.format_date(),
            description: holiday.description,
            is_joint_leave: holiday.is_joint_leave,
        }
    }
}

// GroupedHolidaysDto is used to transfer grouped holidays (e.g., joint and non-joint leave) over the API.
// It contains:
// - joint_leave: A list of holidays that are joint leave.
// - non_joint_leave: A list of holidays that are not joint leave.
#[derive(Serialize)]
pub struct GroupedHolidaysDto {
    pub joint_leave: Vec<HolidayDto>,
    pub non_joint_leave: Vec<HolidayDto>,
}

impl GroupedHolidaysDto {
    // The new() function is a constructor for GroupedHolidaysDto.
    // It takes two vectors of holidays (joint leave and non-joint leave), converts them to DTOs, and returns a GroupedHolidaysDto.
    pub fn new(joint_leave: Vec<Holiday>, non_joint_leave: Vec<Holiday>) -> Self {
        Self {
            joint_leave: joint_leave.into_iter().map(HolidayDto::from).collect(),
            non_joint_leave: non_joint_leave.into_iter().map(HolidayDto::from).collect(),
        }
    }

    // The to_hashmap() function converts the grouped holidays into a HashMap.
    // It groups the holidays by their type (joint leave or non-joint leave) and returns the result as a HashMap.
    pub fn to_hashmap(self) -> HashMap<String, Vec<HolidayDto>> {
        let mut result = HashMap::new();
        result.insert("joint_leave".to_string(), self.joint_leave);
        result.insert("non_joint_leave".to_string(), self.non_joint_leave);
        result
    }
}
