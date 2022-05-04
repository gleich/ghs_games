use chrono::{DateTime, Local};
use fetch::RawEvent;

mod fetch;

fn main() {
    let mut raw_events = RawEvent::fetch_this_weeks().expect("Failed to fetch matches");
    DateTime::parse_from_str("5.8.1994 8:00 am +0000", "%d.%m.%Y %H:%M %P %z")
        .expect("failed to run doc parse");
    println!(
        "{}",
        raw_events
            .get_mut(2)
            .unwrap()
            .clean()
            .expect("Failed to clean event")
            .unwrap()
            .time
            .format("%Y-%m-%d %I:%M %p %z")
    );
}
