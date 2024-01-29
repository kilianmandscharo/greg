#[derive(Clone, Copy, Debug)]
pub struct TimeOfDay {
    pub hour: u16,
    pub minute: u16,
    pub total: u16,
}

impl TimeOfDay {
    pub fn new(hour: u16, minute: u16) -> Self {
        let total = hour * 60 + minute;
        Self {
            hour,
            minute,
            total,
        }
    }

    pub fn from_minutes(minutes: u16) -> Self {
        let hour = minutes / 60;
        let minute = minutes % 60;
        Self {
            hour,
            minute,
            total: minutes,
        }
    }

    pub fn to_string(&self) -> String {
        let hours = if self.hour < 10 {
            format!("0{}", self.hour)
        } else {
            self.hour.to_string()
        };
        let minutes = if self.minute < 10 {
            format!("0{}", self.minute)
        } else {
            self.minute.to_string()
        };
        format!("{hours}:{minutes}")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum StepType {
    Hour(u16),
    Minute(u16),
}

pub struct TimeOfDayRange {
    start: TimeOfDay,
    end: TimeOfDay,
    step_type: StepType,
    current_minute: u16,
}

impl TimeOfDayRange {
    pub fn new(start: TimeOfDay, end: TimeOfDay, step_type: StepType) -> Result<Self, ()> {
        if start.total >= end.total {
            return Err(());
        }

        Ok(Self {
            start,
            end,
            step_type,
            current_minute: start.total,
        })
    }
}

impl Iterator for TimeOfDayRange {
    type Item = TimeOfDay;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_minute > self.end.total {
            return None;
        }

        let item = TimeOfDay::from_minutes(self.current_minute);

        let step = match self.step_type {
            StepType::Hour(n) => n * 60,
            StepType::Minute(n) => n,
        };

        self.current_minute += step;

        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::{StepType, TimeOfDay, TimeOfDayRange};

    #[test]
    fn construct_time_of_day_range() {
        let test_cases = vec![
            (
                TimeOfDayRange::new(
                    TimeOfDay::new(10, 0),
                    TimeOfDay::new(10, 0),
                    StepType::Hour(1),
                ),
                true,
            ),
            (
                TimeOfDayRange::new(
                    TimeOfDay::new(12, 0),
                    TimeOfDay::new(10, 0),
                    StepType::Hour(1),
                ),
                true,
            ),
            (
                TimeOfDayRange::new(
                    TimeOfDay::new(10, 0),
                    TimeOfDay::new(12, 0),
                    StepType::Hour(1),
                ),
                false,
            ),
        ];

        for test in test_cases {
            let (range, should_error) = test;
            assert_eq!(range.is_err(), should_error);
        }
    }

    #[test]
    fn iterate_time_of_day_range() {
        let test_cases = vec![
            (
                TimeOfDayRange::new(
                    TimeOfDay::new(8, 0),
                    TimeOfDay::new(10, 0),
                    StepType::Hour(1),
                ),
                vec!["08:00", "09:00", "10:00"],
            ),
            (
                TimeOfDayRange::new(
                    TimeOfDay::new(8, 0),
                    TimeOfDay::new(10, 15),
                    StepType::Hour(1),
                ),
                vec!["08:00", "09:00", "10:00"],
            ),
            (
                TimeOfDayRange::new(
                    TimeOfDay::new(8, 0),
                    TimeOfDay::new(10, 15),
                    StepType::Hour(2),
                ),
                vec!["08:00", "10:00"],
            ),
            (
                TimeOfDayRange::new(
                    TimeOfDay::new(8, 0),
                    TimeOfDay::new(10, 5),
                    StepType::Minute(30),
                ),
                vec!["08:00", "08:30", "09:00", "09:30", "10:00"],
            ),
            (
                TimeOfDayRange::new(
                    TimeOfDay::new(8, 0),
                    TimeOfDay::new(8, 50),
                    StepType::Minute(15),
                ),
                vec!["08:00", "08:15", "08:30", "08:45"],
            ),
        ];

        for test in test_cases {
            let (range, target) = test;
            let times: Vec<String> = range
                .unwrap()
                .map(|time_of_day| time_of_day.to_string())
                .collect();
            assert_eq!(times, target);
        }
    }
}
