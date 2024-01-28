pub struct TimeOfDay {
    pub hour: u16,
    pub minute: u16,
}

impl TimeOfDay {
    pub fn new(hour: u16, minute: u16) -> Self {
        Self { hour, minute }
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

    pub fn total_minutes(&self) -> u16 {
        self.hour * 60 + self.minute
    }
}
