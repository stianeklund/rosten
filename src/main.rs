/// Rosten
/// Track Bring packages in Rust
/// For testing use "TESTPACKAGE-AT-PICKUPPOINT" as tracking number
/// https://www.mybring.com/tracking/api

#[macro_use]
extern crate rustc_serialize;
extern crate hyper;
extern crate clap;

mod json_response;
use std::io::Read;
use rustc_serialize::json;
use hyper::{Client};
use clap::{Arg, App};
use json_response::BringResponse;


pub fn main() {
    let matches = App::new("Rosten")
        .version("1.0")
        .author("Stian Eklund. <stiane@protonmail.com>")
        .about("Get shipment status of your Bring / Posten packages")
        .arg(Arg::with_name("track")
             .help("Get package status")
             .short("t")
             .long("track")
             .takes_value(true))
        .get_matches();

    let input = matches.value_of("track").unwrap();
    let url = format!("https://tracking.bring.com/tracking.json?q={}", input);
    let url = url.as_str(); // convert &str into a String

    fn get_content(url: &str) -> hyper::Result<String> {
        let client = Client::new();
        let mut response = try!(client.get(url).send());
        let mut buf = String::new();
        try!(response.read_to_string(&mut buf));
        Ok(buf)
    }

    let buf = get_content(url).unwrap();

    fn deserialize(buf: &str) {
        // pass & coerce &String (result of get_content) to json::decode
        let deserialize: BringResponse = json::decode(&buf).unwrap();
        let sets = deserialize.consignmentSet;
        for i in 0..sets.len() {
            let consignment_set = &sets[i];
            for x in 0..consignment_set.packageSet.len() {
                let package_set = &consignment_set.packageSet[x];
                println!("Package number: {}", package_set.packageNumber);
                println!("Sender name: {:?}", package_set.senderName);
                for n in 0..package_set.eventSet.len() {
                    let event_set = &package_set.eventSet[n];
                    println!("Event status: {}", event_set.status);
                    println!("Event description: {}", event_set.description);
                    break;
                 }
             }
         }
    }
    deserialize(&buf);
}
