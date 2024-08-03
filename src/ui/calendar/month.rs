use std::rc::Rc;
use std::cell::RefCell;
use fltk::{
    app::{self, Sender}, button::Button, enums::*, group::Flex, prelude::*, widget_extends
};
use time::{
    Date, 
    Month, 
    OffsetDateTime,
    Weekday
};
use crate::ui::calendar::week;

enum Message {
    NextMonth,
    PrevMnt
}

pub struct MonthGUI {
    wid: Flex,
    month: Rc<RefCell<Month>>,
    year: Rc<RefCell<i32>>,
    weeks: Rc<RefCell<Vec<week::Week>>>
}

impl MonthGUI {
    pub fn default(col: Flex) -> Self {
        let mut wid = col;

        let date = OffsetDateTime::now_local().unwrap();

        //fill the weeks in the month
        let mut weeks = Vec::new();
        let mut day = Date::from_calendar_date(date.year(), date.month(), 1).unwrap();
        while day.month() == date.month() && day.year() == date.year() {
            let mut week = week::Week::new(day);
            week.add(&mut wid);
            weeks.push(week);
            
            // advance to next week

            /*do*/ while {
                day = day.next_day().unwrap();
                day.weekday() != Weekday::Sunday
            } {}
        }

        let month = Rc::from(RefCell::from(date.month()));
        let year = Rc::from(RefCell::from(date.year()));
        let weeks = Rc::from(RefCell::from(weeks));

        let mut weeks_cl = weeks.clone();
        let (s, r) = app::channel::<Message>();
        wid.set_callback(move |wid| {
            if let Some(Message::NextMonth) = r.recv() {
                // add code to handle next button
            }
        });

        // !!!!!!!!!! guide for next prev buttons
        // let mut day = Date::from_calendar_date(date.year(), date.month().next(), 1).unwrap();
        // for week in weeks.iter_mut() {
        //     week.change_week(day);

        //     /*do*/ while {
        //         day = day.next_day().unwrap();
        //         day.weekday() != Weekday::Sunday
        //     } {}
        // }
        wid.begin();
        let mut row = Flex::default().row();
        row.set_callback(|row| {
            row.parent().unwrap().do_callback();
        });
        let prev_btn = MonthButton::new(MonthButtonType::Previous, s.clone());
        let next_btn = MonthButton::new(MonthButtonType::Next, s.clone());
        row.fixed(&prev_btn.wid, 100);
        row.fixed(&next_btn.wid, 100);
        row.end();
        wid.end();
        MonthGUI {
            wid: wid,
            month: month,
            year: year,
            weeks: weeks
        }
    }
}

widget_extends!(MonthGUI, Flex, wid);

// Buttons for MonthGUI
enum MonthButtonType {
    Next,
    Previous
}
struct MonthButton {
    wid: Button,
    b_type: MonthButtonType
}

impl MonthButton {
    fn new(b_type: MonthButtonType, s: Sender<Message>) -> Self {
        let arrow;

        let mut wid = Button::default();

        match b_type {
            MonthButtonType::Next => {
                arrow = '\u{27A1}';
                wid.set_callback(move |but| {
                    s.send(Message::NextMonth);
                    but.parent().unwrap().do_callback();
                });
            }
            MonthButtonType::Previous => {
                arrow = '\u{2B05}';
            }
        }

        let label= format!("{}", arrow);
        wid.set_label(label.as_str());

        MonthButton {
            wid: wid,
            b_type: b_type
        }
    }
}

widget_extends!(MonthButton, Button, wid);