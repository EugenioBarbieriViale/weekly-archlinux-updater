use std::fs::File;
use std::io::prelude::*;
use chrono::Datelike;

fn main() {
    let now = chrono::offset::Local::now();
    let weekday: String = now.date_naive().weekday().to_string();

    let now: String = now.to_string();

    let date = &now[0..10];
    let time = &now[11..19];

    println!("{now}");
    println!("{date}");
    println!("{time}");
    println!("{weekday}");

    let event = date.to_owned() + " - " + time + " - " + &weekday;

    let mut file = File::create("history.log");
    file.write_all(b"{event}");
}
