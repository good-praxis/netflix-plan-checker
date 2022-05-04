use chrono::format::ParseError;
use chrono::NaiveDateTime;
use serde::Deserialize;
use std::env;
use std::fs::File;

#[derive(Deserialize)]
struct Record {
    #[serde(alias = "Profile Name")]
    name: String,

    #[serde(alias = "Source")]
    source: String,

    #[serde(alias = "Navigation Level")]
    nav: String,

    #[serde(alias = "Click Utc Ts")]
    timestamp: String,
}

fn main() {
    let path: String = env::args()
        .nth(1)
        .expect("No path to clickstream file provided");

    if !path.ends_with(".csv") {
        panic!("Not a .csv file");
    }

    let records = read_records(&path);

    println!("{}", records.len());
}

fn read_records(path: &str) -> Vec<Record> {
    let mut records = Vec::new();
    let file = File::open(path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        records.push(result.unwrap());
    }

    records
}
