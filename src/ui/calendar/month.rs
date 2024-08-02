use fltk::{
    enums::*,
    group::Flex,
    prelude::*,
    widget_extends
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
        wid.end();
        MonthGUI {
            wid: wid,
            month: date.month(),
            year: date.year(),
            weeks: weeks
        }
    }
}