use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::from_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::from_minutes(self.hours * 60 + self.minutes + minutes)
    }

    fn from_minutes(total_minutes: i32) -> Clock {
        let modulo_minutes = total_minutes % 1440;

        let final_minutes = if modulo_minutes < 0 {
            modulo_minutes + 1440
        } else {
            modulo_minutes
        };

        Clock {
            hours: final_minutes / 60,
            minutes: final_minutes % 60,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
