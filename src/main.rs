/// Rosten
/// Track Bring packages in Rust
/// For testing use "TESTPACKAGE-AT-PICKUPPOINT" as tracking number
/// https://www.mybring.com/tracking/api

extern crate rustc_serialize;
extern crate hyper;

mod json_response;

use std::io;
use std::io::Read;
use std::fmt::Display;
use rustc_serialize::json;
use hyper::{Client};
use json_response::BringResponse;

pub fn main() {

    // Read user input
    println!("Enter tracking number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    // TODO Implement http query, check hyper docu
    let url = format!("https://tracking.bring.com/tracking.json?q={}", input);
    let url = url.as_str(); // convert &str into a String

    fn get_content(url: &str) -> hyper::Result<String> {
        let client = Client::new();
        let mut response = try!(client.get(url).send());
        let mut buf = String::new();
        try!(response.read_to_string(&mut buf));
        Ok(buf)
    }
    // TODO Figure out how to return &str for json::decode
    let buf = get_content(url);
    let deserialize :BringResponse = json::decode(&buf).unwrap();
    let sets = deserialize.consignmentSet;
    for i in 0..sets.len() {
        let set = &sets[i];
        println!("Sets is {}", set.senderName);
        for x in 0..set.packageSet.len() {
            let package_set = &set.packageSet[x];
            println!("Packageset number is {}", x);
            println!("Status is: {}", package_set.statusDescription);

            }
        }
    }
