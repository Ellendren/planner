pub mod day;
pub mod week_label;
pub mod week;
pub mod week_days;
pub mod month;
pub mod month_year_choice;

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
    month_year_choice: month_year_choice::MonthYearChoice,
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
            month_year_choice: month_year_choice::MonthYearChoice::current(),
            label: label.to_string()
        };

        cal
    }

    // call to add this compnent to a group
    pub fn add(&mut self) {

        self.wid.fixed(self.col_wid.borrow(), 1000);
        self.col_wid.set_pad(10);
        self.col_wid.set_margin(20);

        self.month_year_choice.add().unwrap();

        week_label::WeekLabel(&mut self.col_wid);

        let month = month::MonthGUI::current();
        month.add(&mut self.col_wid).unwrap();

        self.col_wid.end();
        self.wid.end();
    }
}