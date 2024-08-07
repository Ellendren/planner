use fltk::{
    enums::*, 
    group::Flex, 
    macros::table, 
    prelude::*, 
    widget_extends
};
use fltk_table::{SmartTable, TableOpts};

pub struct EventList {
    wid: SmartTable,
    // events:
}

impl EventList {
    pub fn default() -> Self {
        let mut evls = EventList {
            wid: SmartTable::default()
        };
        evls.set_opts(TableOpts {
            editable: true,
            rows: 0,
            cols: 0,
            ..Default::default()
        });
        
        evls.end();

        evls
    }
}

widget_extends!(EventList, SmartTable, wid);