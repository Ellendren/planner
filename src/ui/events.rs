pub mod list;
pub mod list_item;

use fltk::{
    enums::*,
    group::Flex,
    prelude::*,
    widget_extends
};

pub struct Events {
    wid: Flex
}

impl Events {
    pub fn new() -> Self {
        let label = "Events";

        let ev = Events {
            wid: Flex::default()
                .with_label(&label)
                .row()
        };

        ev.end();

        ev
    }

    pub fn list(&self) {
        self.begin();

        list::EventList::default();

        self.end();
    }
}

widget_extends!(Events, Flex, wid);