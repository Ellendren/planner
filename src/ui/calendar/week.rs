use std::fmt::Error;

use fltk::{
    enums::*,
    prelude::*,
    group::Flex
};
use time::{
    Weekday,
    OffsetDateTime,
    Date
};
use colored::Colorize;
use crate::ui::calendar::{day, week_days};

pub struct Week {
    days: Vec<day::DayButton>,
    days_in_week: Vec<Weekday>,
    start_date: Date,
    wid: Flex
}

impl Week {
    // return default week with a start date as current local date
    pub fn default() -> Self {
        let wid = Flex::default().row();

        let days: Vec<day::DayButton> = Vec::new();

        let days_in_week = week_days::days_in_week();

        let start_date = OffsetDateTime::now_local().unwrap().date();

        Week {
            days,
            days_in_week,
            start_date,
            wid
        }
    }

    pub fn new(start_date: Date) -> Self {
        let mut new_week = Week::default();

        new_week.start_date = start_date;

        new_week
    } 

    // if succesful returns the first day of either the next weel or next month
    // whichever one commes first
    pub fn add(mut self, col_id: &mut Flex) -> Result<Date, Error>{
        col_id.fixed(&self.wid, 100);

        let mut curr_weekday_date = self.start_date;
        for weekday in self.days_in_week.iter() {
            let day;
            if *weekday == curr_weekday_date.weekday() && curr_weekday_date.month() == self.start_date.month() {
                day = day::DayButton::new(curr_weekday_date);

                curr_weekday_date = match curr_weekday_date.next_day() {
                    Some(next_day) => next_day,
                    None => {
                        let err_msg = format!("Couldn't get next day for Date: {curr_weekday_date}");

                        eprint!("{}", err_msg.red());
                        return Err(Error);
                    }
                }
            }
            else {
                day = day::DayButton::new_blank(curr_weekday_date);
            }
            self.days.push(day);
        }

        self.wid.end();

        Ok(curr_weekday_date)
    }


}