use std::env;
use std::fs::File;
use std::path::Path;

use chrono::prelude::*;

fn main() {
    let mut args = env::args().skip(1);
    let migration_name = args.next();

    match migration_name {
        Some(s) => {
            let dt = Local::now();

            let minute = format_time(dt.minute());
            let hour = format_time(dt.hour());
            let day = format_time(dt.day());
            let month = format_time(dt.month());
            let year = dt.year().to_string();

            let file_name = format!("{}_{}_{}_{}{}_{}.sql", year, month, day, hour, minute, s);

            if Path::new(file_name.as_str()).exists() {
                println!("File already exists.");
            } else {
                File::create(file_name).unwrap();
                println!("File created successfully.")
            }
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