use chrono::Datelike;

fn main() {
    let now = chrono::offset::Local::now();
    let weekday: String = now.date().weekday().to_string();

    let now: String = now.to_string();

    let date = &now[0..10];
    let time = &now[11..19];

    println!("{now}");
    println!("{date}");
    println!("{time}");
    println!("{weekday}");
}
