use chrono::Datelike;

use std::fs::{File, OpenOptions, read_to_string};
use std::io::Write;
use std::process::Command;

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
    let UPDATE_WEEKDAY = String::from("Sun");

    let filename = "updates_history.log";
    let ev = Event::new();

    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    if ev.weekday == UPDATE_WEEKDAY {
        update_history(ev, filename, &mut file);
    }
}

fn read(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("Can't read file!")
        .lines()
        .map(String::from)
        .collect()
}

fn update_history(ev: Event, filename: &str, file: &mut File) {
    let history: Vec<String> = read(filename);

    if history.len() != 0 {
        let last_ev: String = history.last().unwrap().to_string();
        let last_weekday = &last_ev[24..27];

        if last_ev != ev.to_str() && last_weekday != ev.weekday {
            // update_sys();
            writeln!(*file, "{}", &ev.to_str()).expect("Can't write to file!");
        }
    }
    else {
        writeln!(*file, "{}", &ev.to_str()).expect("Can't write to file!");
    }
}

// THIS DOESNT WORK
// fn update_sys() {
//     Command::new("sudo pacman")    
//         .arg("-Syu")
//         .spawn()
//         .expect("Failed to run sudo pacman -Syu");
// }
