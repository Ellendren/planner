use fltk::{
    enums::*,
    prelude::*,
    group::Flex,
    text::{TextDisplay, TextBuffer},
};
use crate::ui::calendar::week_days;

pub fn WeekLabel(col_id: &mut Flex) {
    let days = Flex::default().row();

    col_id.fixed(&days, 30);

    let days_in_week= week_days::days_in_week();

    for day in days_in_week.iter() {
        let mut day_label = TextDisplay::default();

        day_label.set_color(Color::from_rgb(100, 80, 100));

        let mut buf = TextBuffer::default();
        buf.append(&day.to_string());
        day_label.set_buffer(buf.clone());
    }

    days.end();
}