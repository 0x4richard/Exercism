use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = MINUTES_PER_HOUR * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = (minutes + hours * MINUTES_PER_HOUR).rem_euclid(MINUTES_PER_DAY);

        Self { minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = self.minutes.div_euclid(MINUTES_PER_HOUR);
        let m = self.minutes.rem_euclid(MINUTES_PER_HOUR);
        write!(f, "{:02}:{:02}", h, m)
    }
}
