use fltk::{
    enums::*,
    prelude::*,
    group::Flex,
    text::{TextDisplay, TextBuffer},
};
use time::Weekday;

pub fn WeekLabel(col_id: &mut Flex) {
    let days = Flex::default().row();

    col_id.fixed(&days, 30);

    let week = vec![
        Weekday::Sunday,
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
    ];

    for day in week.iter() {
        let mut day_label = TextDisplay::default();

        day_label.set_color(Color::from_rgb(100, 80, 100));

        let mut buf = TextBuffer::default();
        buf.append(&day.to_string());
        day_label.set_buffer(buf.clone());
    }

    days.end();
}