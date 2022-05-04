use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde::Deserialize;
use std::collections::{BTreeMap, HashSet};
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

    let overlap = find_overlap(&records);

    render_overlap(overlap);
}

fn render_overlap(overlap: BTreeMap<NaiveDate, HashSet<&str>>) {
    for (date, names) in overlap {
        print!("{}: ", date);
        println!("{} concurrent users", names.len());
    }
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

fn find_overlap(records: &Vec<Record>) -> BTreeMap<NaiveDate, HashSet<&str>> {
    let mut overlap: BTreeMap<NaiveDate, HashSet<&str>> = BTreeMap::new();
    let mut prev_ts: (NaiveDateTime, &str) = (NaiveDateTime::from_timestamp(0, 0), "");

    for record in records {
        if record.timestamp.signed_duration_since(prev_ts.0) < Duration::hours(1)
            && record.name != prev_ts.1
        {
            overlap
                .entry(prev_ts.0.date())
                .or_insert_with(HashSet::new)
                .insert(prev_ts.1); // insert previous timestamp to avoid single user overlaps
            overlap
                .entry(record.timestamp.date())
                .or_insert_with(HashSet::new)
                .insert(&record.name);
        }
        prev_ts = (record.timestamp, &record.name);
    }

    overlap
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
