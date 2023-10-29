use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::integral;

pub struct ParseData {
    pub title: String,
    pub rate: String,
    pub unit: String,
    pub time: String,
    pub ns_acc: Vec<f64>, // NS Acceleration
    pub ew_acc: Vec<f64>, // EW Acceleration
    pub ud_acc: Vec<f64>, // UD Acceleration
}

pub fn csv_parse(path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let file = File::open(path)?;
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut data = ParseData {
        title: String::new(),
        rate: String::new(),
        unit: String::new(),
        time: String::new(),
        ns_acc: Vec::new(),
        ew_acc: Vec::new(),
        ud_acc: Vec::new(),
    };

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        if let Some(field) = record.get(0) {
            if field.starts_with("SITE CODE=") {
                data.title = field.to_string();
            } else if field.starts_with("SAMPLING RATE =") {
                data.rate = field.to_string();
            } else if field.starts_with(" UNIT  =") {
                data.unit = field.to_string();
            } else if field.starts_with("INITIAL TIME =") {
                data.time = field.to_string();
            } else if i >= 7 {
                let ns: f64 = record[0].parse()?;
                let ew: f64 = record[1].parse()?;
                let ud: f64 = record[2].parse()?;
                data.ns_acc.push(ns);
                data.ew_acc.push(ew);
                data.ud_acc.push(ud);
            }
        }
    }

    integral::integral(&data);

    Ok(())
}
