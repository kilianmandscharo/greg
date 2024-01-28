use crate::time_of_day::TimeOfDay;

pub struct Block {
    pub start: TimeOfDay,
    pub end: TimeOfDay,
    pub title: String,
    pub description: String,
}

impl Block {
    pub fn new(start: TimeOfDay, end: TimeOfDay, title: String, description: String) -> Self {
        Self {
            start,
            end,
            title,
            description,
        }
    }
}
