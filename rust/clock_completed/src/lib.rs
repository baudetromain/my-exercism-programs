use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub struct Clock
{
    hours: i32,
    minutes: i32
}

impl Clock
{
    pub fn new(hours: i32, minutes: i32) -> Self
    {
        let right_hours = hours.rem_euclid(24) + minutes / 60 - (if minutes < 0 && minutes.rem_euclid(60) != 0 {1} else {0});
        let right_minutes = minutes.rem_euclid(60);

        Clock
        {
            hours: right_hours.rem_euclid(24),
            minutes: right_minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self
    {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}