use fltk::{
    enums::*,
    group::Flex,
    prelude::*,
    widget_extends,
    button::Button
};
use time::{
    Date, 
    Month, 
    OffsetDateTime,
    Weekday
};
use crate::ui::calendar::week;

pub struct MonthGUI {
    wid: Flex,
    month: Month,
    year: i32,
    weeks: Vec<week::Week>
}

impl MonthGUI {
    pub fn default(col: Flex) -> Self {
        let mut wid = col;

        let date = OffsetDateTime::now_local().unwrap();

        //fill the weeks in the month
        let weeks = Vec::new();
        let mut day = Date::from_calendar_date(date.year(), date.month(), 1).unwrap();
        while day.month() == date.month() && day.year() == date.year() {
            let week = week::Week::new(day);
            week.add(&mut wid);
            
            // advance to next week

            /*do*/ while {
                day = day.next_day().unwrap();
                day.weekday() != Weekday::Sunday
            } {}
        }
        wid.begin();
        let mut row = Flex::default().row();
        let prev_btn = MonthButton::new(MonthButtonType::Previous);
        let next_btn = MonthButton::new(MonthButtonType::Next);
        row.fixed(&prev_btn.wid, 100);
        row.fixed(&next_btn.wid, 100);
        row.end();
        wid.end();
        MonthGUI {
            wid: wid,
            month: date.month(),
            year: date.year(),
            weeks: weeks
        }
    }
}

widget_extends!(MonthGUI, Flex, wid);

// Buttons for MonthGUI
enum MonthButtonType {
    Next,
    Previous
}
struct MonthButton {
    wid: Button,
    b_type: MonthButtonType
}

impl MonthButton {
    fn new(b_type: MonthButtonType) -> Self {
        let arrow;

        match b_type {
            MonthButtonType::Next => {
                arrow = '\u{27A1}';
            }
            MonthButtonType::Previous => {
                arrow = '\u{2B05}';
            }
        }

        let label= format!("{}", arrow);

        MonthButton {
            wid: Button::default().with_label(label.as_str()),
            b_type: b_type
        }
    }
}

widget_extends!(MonthButton, Button, wid);