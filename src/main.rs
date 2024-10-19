use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::Datelike;

struct Event {
    date: String,
    time: String,
    weekday: String,
}

impl Event {
    fn new() -> Self {
        let now = chrono::offset::Local::now();
        let weekday: String = now.date_naive().weekday().to_string();

        let now: String = now.to_string();

        let date = now[0..10].to_string();
        let time = now[11..19].to_string();

        Self {
            date,
            time,
            weekday,
        }
    }

    fn event_str(&self) -> String {
        self.date.clone() + " - " + &self.time + " - " + &self.weekday
    }
}

fn main() {
    let filename = "history.log";
    let ev = Event::new();

    handle_history(&ev.event_str(), filename);
}

fn handle_history(event: &String, filename: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .expect("Can't open file!");

    writeln!(file, "{}", event).expect("Can't write to file");
}
