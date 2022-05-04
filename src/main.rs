use chrono::DateTime;
use fetch::RawEvent;

mod fetch;

fn main() {
    RawEvent {
        postponed: String::from("0"),
        month: String::from("5"),
        year: String::from("2022"),
        day: String::from("3"),
        location: String::from("Sanborn Regional High School"),
        event_type: String::from("sport"),
        opponent: String::from("Multiple Opponents"),
        cancelled: String::from("0"),
        name: String::from("Boys-Girls Varsity Outdoor Track"),
        home: String::from("0"),
        time: String::from("4:00 PM"),
        rescheduled_date: String::from(""),
    }
    .clean()
    .expect("Failed to do the test");
    let mut raw_events = RawEvent::fetch_this_weeks().expect("Failed to fetch matches");
    DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")
        .expect("failed to run doc parse");
    dbg!(raw_events
        .get_mut(2)
        .unwrap()
        .clean()
        .expect("Failed to clean event")
        .unwrap());
}
