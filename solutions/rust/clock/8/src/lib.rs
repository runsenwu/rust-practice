use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut temp_hour = hours;
        let mut temp_minutes = minutes;
        let extra_hour = minutes % 60;
        
        if (minutes < 0) {
            temp_minutes = 60 - (-1 * minutes) % 60;
        }
        if (hours < 0) {
            temp_hour = ((24 - (-1 * hours) % 24) + extra_hour) % 24;
        }

        Clock {
            minutes: temp_minutes % 60,
            hours: (temp_hour + extra_hour) % 24,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let extra_hour = (self.minutes + minutes) / 60;
        Clock {
            minutes: {
                (self.minutes + minutes) % 60
            },
            hours: (self.hours + extra_hour + 24) % 24,
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