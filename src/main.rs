#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate clap;
extern crate reqwest;

mod json_response;
use std::io::Read;
use clap::{App, Arg, AppSettings};
use json_response::{BringResponse, Json, Eventset, Packageset, Consignmentset};

pub fn main() {
    let matches = App::new("Rosten")
        .version("1.1")
        .author("Stian Eklund. <stiane@protonmail.com>")
        .about("Get shipment status of your Bring & Posten packages")
        .setting(AppSettings::ArgRequiredElseHelp) // display help text if no args present at runtime
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .help("Get package status")
            .takes_value(true))
        .get_matches();

    let input = matches.value_of("track").unwrap();
    let url = format!("https://tracking.bring.com/tracking.json?q={}", input);
    let url = url.as_str();

    fn get_content(url: &str) -> Result<String, reqwest::Error> {
        let mut result = String::new();
        let mut response = reqwest::get(url)?.read_to_string(&mut result);
        Ok(result)
    }

    let buf = get_content(url).unwrap();

    // TODO Improve on this as it's a bit too nested currently
    fn deserialize(buf: &str) {
        let deserialized: Result<BringResponse, serde_json::Error> = serde_json::from_str(&buf);
        match deserialized {
            Ok(deserialized) => {
                let sets = deserialized.consignmentSet;
                for i in 0..sets.len() {
                    let consignment_set = &sets[i];

                    for x in 0..consignment_set.packageSet.len() {
                        let package_set = &consignment_set.packageSet[x];

                        match consignment_set.packageSet[x] {
                            Packageset {
                                productName: Some(ref productName),
                                packageNumber: Some(ref packageNumber),
                                ..
                            } => println!("Product Name: {}\nPackage number: {}", productName, packageNumber),
                            _ => println!("Not covered"),
                        }
                        for n in 0..package_set.eventSet.len() {
                            match package_set.eventSet[n] {
                                Eventset {
                                    description: Some(ref description),
                                    status: Some(ref status), ..
                                } => println!("Description: {}\nStatus: {}", description, status),
                                _ => println!("Not covered"),
                            }
                        }
                    }
                }
            },
            Err(_) => deserialize_err(&buf)
        }
    }

    fn deserialize_err(buf: &str) {
        let deserialized: Result<Json, serde_json::Error> = serde_json::from_str(&buf);
        match deserialized {
            Ok(deserialized) => {
                let set = deserialized.consignmentSet;
                println!("Check your tracking number. {:?}, code: {:?}", &set[0].error.message, &set[0].error.code);
            },
            Err(_) => println!("Error deserializing error response")
        }
    }
    deserialize(&buf);
}
