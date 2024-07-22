use time::{Date, Month, OffsetDateTime, Weekday};
use crate::events::{self, event::{self, Event}};

#[derive(Debug)]
pub struct Day {
    date: Date,
    events: Vec<Event>
}

impl Day {
    pub fn new(date: Date, events: Vec<Event>) -> Self {
        Day {
            date,
            events,
        }
    }
    
    //makes a new day for today
    pub fn default() -> Self {
        Day {
            date: OffsetDateTime::now_local().unwrap().date(),
            events: Vec::new()
        }
    }
    
    pub fn date(&self) -> String {
        let month = &self.date.month().to_string()[..3];
        let day_of_month = self.date.day();

        let diplay_date = format!("{month} {day_of_month}");

        diplay_date
    }
}