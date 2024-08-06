use fltk::{
    button::Button, 
    enums::*, 
    prelude::*, 
    widget_extends
};
use time::Date;
use crate::calendar::day::Day;
use crate::ui::calendar::day_popup;


pub struct DayButton {
    wid: Button,
    day: Day
}

impl DayButton {
    pub fn default() -> Self {
        let day = Day::default();

        DayButton {
            wid: Button::default().with_label(&day.date()),
            day: day
        }
    }

    pub fn new(date: Date) -> Self {
        let day = Day::new(date, Vec::new());

        let day_button = DayButton {
            wid: Button::default().with_label(&day.date()),
            day: day
        };

        day_button.callback()
    }

    pub fn new_blank(date: Date) -> Self {
        let day = Day::new(date, Vec::new());

        DayButton {
            wid: Button::default(),
            day: day
        }
    }

    fn callback(mut self) -> Self{
        let day = self.day.date().clone();
        self.set_callback(move |d| {
            day_popup::DayPopup::new(&day);
        });

        self
    }

    pub fn update_day(&self, date: Date) -> Self {
        let day = Day::new(date, Vec::new());
        let wid = self.wid.clone();
        DayButton {
            wid: wid.with_label(day.date().as_str()),
            day: day
        }
    }

    pub fn update_day_blank(&self, date: Date) -> Self {
        let day = Day::new(date, Vec::new());
        let wid = self.wid.clone();

        DayButton {
            wid: wid.with_label(""),
            day: day
        }
    }
}

widget_extends!(DayButton, Button, wid);