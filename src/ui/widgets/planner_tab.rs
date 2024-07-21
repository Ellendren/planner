use fltk::{
    enums::*, 
    group::{
        Tabs,
        Flex
    },
    prelude::{
        GroupExt, 
        MenuExt, 
        WidgetBase, 
        WidgetExt
    }
};
use crate::ui::calendar::Calendar;

pub struct PlannerTab {
    wid: Tabs,
    calendar: Calendar,
}

impl  PlannerTab {
    pub fn new() -> Self{
        let wid = Tabs::default_fill();

        let mut tab = PlannerTab {
            wid,
            calendar: Calendar::new(),
        };
        
        tab.calendar.add();

        tab.wid.end();
        tab.wid.auto_layout();

        tab
    }
}