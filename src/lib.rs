use chrono::prelude::*;
use csv::Reader;
use std::fs::File;
use std::io::{BufReader, Error};

pub fn now(path: String) -> Result<(), Error> {
    let mut reader = read_csv_file(path).unwrap();
    let ts = get_local_time();

    for record in reader.records() {
        let record = record?;

        let start_time = NaiveTime::parse_from_str(&record[0], "%H:%M:%S").unwrap();
        let end_time = NaiveTime::parse_from_str(&record[1], "%H:%M:%S").unwrap();

        let range = start_time..end_time;
        if range.contains(&ts) {
            println!("{} {} {}", &record[0], &record[1], &record[3]);
            break;
        }
    }

    Ok(())
}

pub fn today(path: String) -> Result<(), Error> {
    let mut reader = read_csv_file(path).unwrap();

    for record in reader.records() {
        let record = record?;
        println!("{} {} {}", &record[0], &record[1], &record[3]);
    }

    Ok(())
}

fn read_csv_file(path: String) -> Result<Reader<BufReader<File>>, Error> {
    let fd = File::open(path)?;
    let buf = BufReader::new(fd);
    let reader = csv::Reader::from_reader(buf);

    return Ok(reader);
}

fn get_local_time() -> NaiveTime {
    let dt = Local::now();
    let ts = dt.time();
    return ts;
}
