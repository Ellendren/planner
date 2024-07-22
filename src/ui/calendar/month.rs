use time::{
    Date, Error, Month, OffsetDateTime
};
use fltk::group::Flex;
use crate::ui::calendar::week::Week;

pub struct MonthGUI {
    year: i32,
    month: Month
}

impl MonthGUI {
    pub fn current() -> Self {
        let curr_date = OffsetDateTime::now_local().unwrap();

        MonthGUI {
            year: curr_date.year(),
            month: curr_date.month()
        }
    }

    pub fn add(self, col_id: &mut Flex) -> Result<(), Error>{
        let mut start_date = Date::from_calendar_date(self.year, self.month, 1)?;

        while start_date.month() == self.month {
            let curr_week = Week::new(start_date);

            start_date = curr_week.add(col_id).unwrap();
        }

        Ok(())
    }
}