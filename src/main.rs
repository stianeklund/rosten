/// Rosten
/// Track Bring packages in Rust
/// For testing use "TESTPACKAGE-AT-PICKUPPOINT" as tracking number
/// https://www.mybring.com/tracking/api

extern crate rustc_serialize;
extern crate hyper;

mod json_response;

use std::io;
use std::io::Read;
use rustc_serialize::json;
use hyper::{Client};
use json_response::BringResponse;

pub fn main() {
    // TODO Get tracking numbers from external file?
    // Crontab process to poll every hour?

    let mut input = String::new();
    println!("Enter tracking number:");
    io::stdin().read_line(&mut input);
    // let input = "TESTPACKAGE-AT-PICKUPPOINT";

    // TODO Look up Hyper query. Difference between this solution?
    let url = format!("https://tracking.bring.com/tracking.json?q={}", input);
    let url = url.as_str(); // convert &str into a String

    fn get_content(url: &str) -> hyper::Result<String> {
        let client = Client::new();
        let mut response = try!(client.get(url).send());
        let mut buf = String::new();
        try!(response.read_to_string(&mut buf));
        Ok(buf)
    }
    // TODO Perhaps wrap deserializing into own function?
    let buf = get_content(url).unwrap();
    // Here we pass & coerce &String (result of get_content) to json::decode
    let deserialize: BringResponse = json::decode(&buf).unwrap();
    let sets = deserialize.consignmentSet;
    // TODO Improve on for loop or look into alternative ways to do this.
    for i in 0..sets.len() {
        let consignment_set = &sets[i];
        for x in 0..consignment_set.packageSet.len() {
            let package_set = &consignment_set.packageSet[x];
            println!("Package number: {}", package_set.packageNumber);
            println!("Sender name: {:?}", package_set.senderName);
            println!("Package Status: {}", package_set.statusDescription);
            for n in 0..package_set.eventSet.len() {
                let event_set = &package_set.eventSet[n];
                println!("Event status: {}", event_set.status);
                println!("Event description: {}", event_set.description);
            }
        }
    }
}
