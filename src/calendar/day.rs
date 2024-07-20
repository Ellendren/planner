use time::{Date, Month};
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
            events
        }
    }
}