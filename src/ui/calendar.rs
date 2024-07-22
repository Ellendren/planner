pub mod day;
pub mod week_label;
pub mod week;
pub mod week_days;

use std::borrow::Borrow;

use fltk::{
    enums::*,
    group::Flex,
    prelude::*
};
use time::{Date, Month};

pub struct Calendar {
    wid: Flex,
    col_wid: Flex,
    label: String
}

impl Calendar {
    pub fn new() -> Self {
        let label = "Calendar";

        let cal = Calendar {
            wid: Flex::default()
                .with_label(format!("{label}\t").as_str())
                .row(),
            col_wid: Flex::default().column(),
            label: label.to_string()
        };

        cal
    }

    // call to add this compnent to a group
    pub fn add(&mut self) {

        self.wid.fixed(self.col_wid.borrow(), 1000);
        self.col_wid.set_pad(10);
        self.col_wid.set_margin(20);
        
        week_label::WeekLabel(&mut self.col_wid);

        let week = week::Week::new(Date::from_calendar_date(2024, Month::July, 1).unwrap());
        week.add(&mut self.col_wid);

        self.col_wid.end();
        self.wid.end();
    }
}