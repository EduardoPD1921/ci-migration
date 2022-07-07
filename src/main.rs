use std::env;
use chrono::prelude::*;

fn main() {
    let mut args = env::args().skip(1);
    let migration_name = args.next();

    match migration_name {
        Some(_s) => {
            let dt = Local::now();

            let hour = format_time(dt.hour());
            let minute = format_time(dt.minute());

            println!("{}{}", hour, minute);
        }
        None => {
            println!("Insert your migration name.");
        }
    }
}


fn format_time(time: u32) -> String {
    let formatted_time = if time < 10 { format!("0{}", time.to_string()) } else { time.to_string() };
    formatted_time
}