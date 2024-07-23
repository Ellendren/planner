use fltk::{
    enums::*, 
    group::Flex, 
    menu::{
        Choice,
        MenuItem
    },
    prelude::*
};
use time::{Month, OffsetDateTime};

pub struct MonthYearChoice {
    row: Flex,
    year: Choice,
    month: Choice,
    max_year: i32,
    min_year: i32
}

impl MonthYearChoice {
    // default has all twelve months and the next ten years
    pub fn default() -> Self {
        let row = Flex::default().row();

        let mut month = Month::January;
        let mut month_choice = Choice::default();
        while month != Month::December {
            month_choice.add_choice(&month.to_string());

            month = month.next();
        }

        let min_year = OffsetDateTime::now_local().unwrap().year();
        let max_year = min_year + 10;
        let mut year_choice = Choice::default();
        for year in min_year..max_year+1 {
            year_choice.add_choice(&year.to_string());
        }

        MonthYearChoice {
            row: row,
            year: year_choice,
            month: month_choice,
            max_year: max_year,
            min_year: min_year
        }
    }

    pub fn current() -> Self {
        let mut choice = MonthYearChoice::default();
        
        //set the current year
        let item = OffsetDateTime::now_local()
            .unwrap()
            .year()
            .to_string()
            .leak();
        let select_year = MenuItem::new(&[item]);
        choice.year.set_item(&select_year);

        //set the current month
        let item= OffsetDateTime::now_local()
            .unwrap()
            .month()
            .to_string()
            .leak();
        let select_month = MenuItem::new(&[item]);

        choice.month.set_item(&select_month);

        choice
    }

    pub fn add(&mut self) -> Result<(), FltkError>{

        self.row.end();

        Ok(())
    }
}