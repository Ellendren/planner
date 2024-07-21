use fltk::{
    button::Button,
    enums::*,
    prelude::*
};
use crate::calendar::day::Day;

pub struct DayButton {
    wid: Button,
    day: Day
}

impl DayButton {
    pub fn new() -> Self {
        let day = Day::default();
        let weekday = day.weekday().to_string();

        DayButton {
            wid: Button::default().with_label(&weekday),
            day: day
        }
    }
}