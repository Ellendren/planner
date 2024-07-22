use fltk::{
    button::Button,
    enums::*,
    prelude::*
};
use time::Date;
use crate::calendar::day::Day;

pub struct DayButton {
    wid: Button,
    day: Day
}

impl DayButton {
    pub fn default() -> Self {
        let day = Day::default();

        DayButton {
            wid: Button::default().with_label(&day.date()),
            day: day
        }
    }

    pub fn new(date: Date) -> Self {
        let day = Day::new(date, Vec::new());

        DayButton {
            wid: Button::default().with_label(&day.date()),
            day: day
        }
    }

    pub fn new_blank(date: Date) -> Self {
        let day = Day::new(date, Vec::new());

        DayButton {
            wid: Button::default(),
            day: day
        }
    }
}