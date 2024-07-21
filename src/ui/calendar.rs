pub mod day;
pub mod week_label;

use std::borrow::Borrow;

use fltk::{
    enums::*,
    group::Flex,
    prelude::*
};

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
        self.col_wid.set_pad(20);
        self.col_wid.set_margin(20);
        // let day = day::DayButton::new();
        week_label::WeekLabel(&mut self.col_wid);

        self.col_wid.end();
        self.wid.end();
    }
}