use std::{borrow::BorrowMut, rc::Rc};
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
    PrevMonth
}

pub struct MonthGUI {
    wid: Flex,
    month: Rc<RefCell<Month>>,
    year: Rc<RefCell<i32>>
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

        let month_cb = month.clone();
        let year_cb = year.clone();
        let (s, r) = app::channel::<Message>();
        wid.set_callback(move |wid| {
            // let weeks = weeks_cb.borrow_mut();
            let mut new_month = *month_cb.borrow();
            let mut new_year = *year_cb.borrow();

            // handle respone
            let response = r.recv();
            match response {
                Some(msg) => {
                    match msg {
                        Message::NextMonth => {
                            new_month = new_month.next();
                            month_cb.swap(&RefCell::from(new_month));
                            if new_month == Month::January {
                                new_year = new_year+1;
                                year_cb.swap(&RefCell::from(new_year)); 
                            }
                        }
                        Message::PrevMonth => {
                            new_month = new_month.previous();
                            month_cb.swap(&RefCell::from(new_month));
                            if new_month == Month::December {
                                new_year = new_year-1;
                                year_cb.swap(&RefCell::from(new_year)); 
                            }
                        }
                    }
                }
                None => {}
            }

            //update weeKs
            let mut start_date = Date::from_calendar_date(new_year, new_month, 1).unwrap();
            for week in weeks.iter_mut() {
                week.change_week(start_date);

                /*do*/ while {
                    start_date = start_date.next_day().unwrap();
                    start_date.weekday() != Weekday::Sunday
                } {}
            }
        });

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
                wid.set_callback(move |but| {
                    s.send(Message::PrevMonth);
                    but.parent().unwrap().do_callback();
                });
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