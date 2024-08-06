pub mod day;
pub mod day_popup;
pub mod week_label;
pub mod week;
pub mod week_days;
pub mod month;

use fltk::{
    enums::*,
    group::Flex,
    prelude::*,
    widget_extends
};

pub struct Calendar {
    wid: Flex,
    label: String
}

impl Calendar {
    pub fn new() -> Self {
        let label = "Calendar";

        let cal = Calendar {
            wid: Flex::default()
                .with_label(format!("{label}").as_str())
                .row(),
            label: label.to_string()
        };

        cal.end();

        cal
    }

    fn show_today(&self) {
        day::DayButton::default();
    }

    pub fn show_month(&mut self) {
        self.begin();

        let mut col = CalendarCol::default_col();
        self.fixed(&col.wid, 1000);

        week_label::week_label(&mut col.wid);

        let month = month::MonthGUI::default(col.wid);
        self.end();
    }
}

widget_extends!(Calendar, Flex, wid);



// colums widge for calander tab
struct CalendarCol {
    wid: Flex
}

impl  CalendarCol{
    fn default_col() -> Self{
        let mut col = CalendarCol{
            wid: Flex::default().column()
        };

        col.set_pad(10);
        col.set_margin(20);

        col
    }
}

widget_extends!(CalendarCol, Flex, wid);