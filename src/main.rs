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

    fn to_str(&self) -> String {
        self.date.clone() + " - " + &self.time + " - " + &self.weekday
    }
}

fn main() {
    let filename = "history.log";
    let ev = Event::new();
    history::handle(ev, filename);
}

pub mod history {
    use std::fs::{OpenOptions, read_to_string};
    use std::io::Write;
    use crate::Event;

    fn write(event: &String, filename: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filename)
            .expect("Can't open file!");

        writeln!(file, "{}", event).expect("Can't write to file!");
    }

    fn read(filename: &str) -> Vec<String> {
        read_to_string(filename)
            .expect("Can't read file!")
            .lines()
            .map(String::from)
            .collect()
    }

    pub fn handle(ev: Event, filename: &str) {
        let last_ev: String = read(filename).last().expect("History out of index").to_string();
        if last_ev != ev.to_str() && last_ev[24..27] != ev.weekday {
            println!("UPDATEW");
            // println!("{:?}", last_ev[24..27]);
            write(&ev.to_str(), filename);
        }
    }
}
