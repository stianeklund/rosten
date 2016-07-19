/// Rosten
/// Track Bring packages in Rust
/// For testing use "TESTPACKAGE-AT-PICKUPPOINT" as tracking number
/// https://www.mybring.com/tracking/api

extern crate rustc_serialize;
extern crate hyper;

mod json_response;

use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use rustc_serialize::json;
use hyper::{Client};
use json_response::BringResponse;

pub fn main() {
    let client = Client::new();
    let url = "https://www.mybring.com/tracking/api";

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
    let deserialize :BringResponse = json::decode(&buffer).unwrap();
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
