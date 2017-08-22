#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate clap;
extern crate reqwest;

mod json_response;
use std::io::Read;
use clap::{App, Arg, AppSettings};
use json_response::{BringResponse, Eventset, Packageset, ErrorConsignmentSet};

pub fn main() {
    let matches = App::new("Rosten")
        .version("1.1")
        .author("Stian Eklund. <stiane@protonmail.com>")
        .about("Get shipment status of your Bring & Posten packages")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("track")
            .short("t")
            .long("track")
            .help("Get package status")
            .takes_value(true))
        .get_matches();

    let input = matches.value_of("track").unwrap();
    let url = format!("https://tracking.bring.com/tracking.json?q={}", input);
    let url = url.as_str();

    fn get_content(url: &str) -> Result<String, Box<std::error::Error>> {
        let mut result = String::new();
        reqwest::get(url)?.read_to_string(&mut result)?;
        Ok(result)
    }

    let buf = get_content(url).unwrap();

    fn deserialize(buf: &str) {
        let deserialized: Result<BringResponse, serde_json::Error> = serde_json::from_str(&buf);

        match deserialized {
            Ok(deserialized) => {
                let sets = deserialized.consignment_set;
                for i in 0..sets.len() {
                    let consignment_set = &sets[i];

                    for x in 0..consignment_set.package_set.len() {
                        let package_set = &consignment_set.package_set[x];


                        match consignment_set.package_set[x] {
                            Packageset {
                                product_name: Some(ref product_name),
                                package_number: Some(ref package_number),
                                ..
                            } => println!("Product Name: {}\nPackage number: {}", product_name, package_number),
                            _ => println!("Not covered"),
                        }
                        for n in 0..package_set.event_set.len() {
                            match package_set.event_set[n] {
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
            Err(_) => deserialize_err(buf),
            }
        }

    fn deserialize_err(buf: &str) {
        let deserialized: Result<ErrorConsignmentSet, serde_json::Error> = serde_json::from_str(&buf);
        match deserialized {
            Ok(deserialized) => {
                println!("Error: {}, Code:{}", deserialized.error.message, deserialized.error.code);
            }
            Err(_) => println!("Error deserializing error response"),
        }
    }
    deserialize(&buf);
}
