use std::fmt;

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let c = Clock { hours, minutes };
        c.add_minutes(0)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        let offset = if (self.minutes + minutes) < 0 { -1 } else { 0 };
        let hours_from_mins = (offset + (self.minutes + minutes) / 60) % 24;
        let hours = (24 + self.hours % 24 + hours_from_mins) % 24;
        let minutes = (60 + self.minutes % 60 + minutes % 60) % 60;
        Clock { hours, minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
