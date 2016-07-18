/// Rosten
/// Track Bring packages in Rust
/// For testing use "TESTPACKAGE-AT-PICKUPPOINT" as tracking number
/// https://www.mybring.com/tracking/api

extern crate curl;
extern crate clap;
extern crate serde_json;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use curl::easy::Easy;
use std::fmt::Display;
use serde_json::Value;

// TODO Define values corresponding to json file
// Would an enum be better here?

// TODO Depending on size, perhaps move structs to different file?

struct Data {
    Firstname: String,
    Lastname: String,
    Age: u32,
    Address: Address,
    PhoneNumber: Vec<String>
}
// Do we need to define everything in tracking.json?
// Is it enough to assign whatever matches to needed values?
struct Address {
    Street: String,
    City: String,
    Country: String
}

pub fn main() {

    // TODO Figure out ~/ path so compiler can find the file easily
    let path = Path::new("/home/stian/projects/rosten/tracking.json");
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(e) => panic!("unable to open {}: {}", display, e.description()),
        Ok(file) => file,
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    // TODO Better names
    let data: Value = serde_json::from_str(&buffer).unwrap();

    // TODO Iterate over consignmentSet
    let values = data.as_object()
        .and_then(|object| object.get("consignmentSet"))
        .and_then(|values| values.as_object())
        .unwrap_or_else(|| {
            panic!("Failed to get object value from json"); // fails at runtime here
        });

    for (key, value) in values.iter() {
        let results = value.find("consignmentId")
            .and_then(|value| value.as_object())
            .unwrap_or_else(|| {
                panic!("Failed to get value from within values");
            });
        println!("{} -> {:?}", key, results);
    }
}
