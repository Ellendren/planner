mod calendar;
mod events;
mod ui;
use time::OffsetDateTime;

fn main() {
    // let curr_datetime = OffsetDateTime::now_local().unwrap();
    // let job_app = events::job_app::JobApp::new_app(
    //     "position".to_string(),
    //      "company".to_string(), 
    //      curr_datetime, 
    //      curr_datetime,
    //      "platform".to_string(), 
    //      "job_description".to_string()
    // );

    // let mut events: Vec<events::event::Event> = Vec::new();
    // let mut dates = Vec::new();
    // let curr_date = curr_datetime.date();
    
    // dates.push(curr_date);
    // events.push(events::event::Event::new(dates, events::event::EventType::JobApp { val: job_app }));
    
    // let today = calendar::day::Day::new(curr_date, events);

    // println!("{:?}", today);
    ui::app::run();
}
