/// Rosten
/// Track Bring packages in Rust
/// For testing use "TESTPACKAGE-AT-PICKUPPOINT" as tracking number
/// https://www.mybring.com/tracking/api

extern crate rustc_serialize;
extern crate hyper;

mod json_response;

use std::io::Read;
use rustc_serialize::json;
use hyper::{Client};
use json_response::BringResponse;

pub fn main() {

    // TODO Implement query
    // Wrap in function?
    let url = "https://tracking.bring.com/tracking.json?q=TESTPACKAGE-AT-PICKUPPOINT";
    let client = Client::new();
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(_) => panic!("Failed to get http response"),
    };
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(buf) => buf,
        Err(_) => panic!("Failed to read to buffer"),
    };

    let deserialize :BringResponse = json::decode(&buf).unwrap();
    let sets = deserialize.consignmentSet;
    // Iterate over consignmentSet and get package status description
    for i in 0..sets.len() {
        let set = &sets[i];
        println!("Sets is {}", set.senderName);
        for x in 0..set.packageSet.len() {
            let package_set = &set.packageSet[x];
            println!("Packageset number is {}", x);
            println!("Packageset is {}", package_set.statusDescription);
        }
    }
}
