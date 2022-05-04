use chrono::NaiveDateTime;
use serde::Deserialize;
use std::env;
use std::fs::File;

#[derive(Deserialize, Debug)]
struct Record {
    #[serde(alias = "Profile Name")]
    name: String,

    #[serde(alias = "Navigation Level")]
    nav: String,

    #[serde(alias = "Click Utc Ts", deserialize_with = "ts_deserializer")]
    timestamp: NaiveDateTime,
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
        let result: Record = result.unwrap();

        if result.nav == "playback" || result.nav == "postPlay" || result.nav == "progressSpinner" {
            records.push(result);
        }
    }

    records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

    records
}

fn ts_deserializer<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
        .map_err(|e| serde::de::Error::custom(format!("{}", e)))?;
    Ok(dt)
}
