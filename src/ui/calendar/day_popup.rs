use fltk::{
    enums::*,
     prelude::*, 
     widget_extends, 
     window::*
};
use time::Date;

pub struct DayPopup {
    win: DoubleWindow
}

impl DayPopup {
    pub fn new(day: &str) -> Self {
        let mut win = Window::new(0, 0, 250, 250, "")
            .with_label(day)
            .center_screen();

        win.end();
        win.show();

        DayPopup {
            win
        }
    }
}

