use std::fmt;

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Clock {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        self::Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.minutes / HOUR, self.minutes % HOUR)
    }
}
