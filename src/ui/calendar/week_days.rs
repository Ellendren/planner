use time::Weekday;

pub fn days_in_week() -> Vec<Weekday> {
    vec![
        Weekday::Sunday,
        Weekday::Monday,
        Weekday::Tuesday,
        Weekday::Wednesday,
        Weekday::Thursday,
        Weekday::Friday,
        Weekday::Saturday,
    ]
}