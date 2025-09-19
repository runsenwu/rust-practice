use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: minutes,
            hours: hours,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let extra_hour = (self.minutes + minutes) / 60;
        Clock {
            minutes: {
                (self.minutes + minutes) % 60
            },
            hours: (self.hours + extra_hour) % 24,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}