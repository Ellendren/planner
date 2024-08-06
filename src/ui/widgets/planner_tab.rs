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
use crate::ui::events::Events;

pub struct PlannerTab {
    wid: Tabs,
    calendar: Calendar,
    events: Events
}

impl PlannerTab {
    pub fn new() -> Self{
        let wid = Tabs::default_fill();

        let mut tab = PlannerTab {
            wid,
            calendar: Calendar::new(),
            events: Events::new()
        };
        
        tab.calendar.show_month();
        tab.events.list();

        tab.end();
        tab.auto_layout();

        tab
    }
}

widget_extends!(PlannerTab, Tabs, wid);