use super::job_app::JobApp;
use time::Date;

#[derive(Debug)]
pub struct Event {
    dates: Vec<Date>,
    event_type: EventType
}

#[derive(Debug)]
pub enum EventType {
    JobApp {
        val: JobApp
    }
}

impl Event {
    pub fn new(dates: Vec<Date>, event_type: EventType) -> Self {
        Event {
            dates,
            event_type
        }
    }
}