use fltk::{
    enums::*, 
    group::{
        Flex, Tabs
    },
    prelude::{
        GroupExt, 
        MenuExt, 
        WidgetBase, 
        WidgetExt
    }, widget_extends
};
use crate::{calendar, ui::calendar::Calendar};

pub struct PlannerTab {
    wid: Tabs,
    calendar: Calendar,
}

impl PlannerTab {
    pub fn new() -> Self{
        let wid = Tabs::default_fill();

        let mut tab = PlannerTab {
            wid,
            calendar: Calendar::new(),
        };
        
        tab.calendar.show_month();

        tab.wid.end();
        tab.wid.auto_layout();

        tab
    }
}

widget_extends!(PlannerTab, Tabs, wid);