use std::fmt;

#[derive(Debug)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_minutes = hours * 60 + minutes;
        if total_minutes < 0 {
            total_minutes = 1440 - ((-total_minutes) % 1440);
        }

        let final_minutes = total_minutes % 60;
        let final_hours = (total_minutes / 60) % 24;
        Clock {
            minutes: final_minutes,
            hours: final_hours,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // we can do the same thing, but what if we can shorten it instead?
        let mut total_minutes = self.hours * 60 + self.minutes + minutes;
        if total_minutes < 0 {
            total_minutes = 1440 - ((-total_minutes) % 1440);
        }

        let final_minutes = total_minutes % 60;
        let final_hours = (total_minutes / 60) % 24;
        Clock {
            minutes: final_minutes,
            hours: final_hours,
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
