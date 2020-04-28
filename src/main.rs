#[macro_use]
extern crate serde_derive;
extern crate clap;
extern crate hyper;
extern crate reqwest;
extern crate serde_json;

mod json_response;

use crate::json_response::{BringResponse, ErrorConsignmentSet, Eventset, Packageset};
use clap::{App, AppSettings, Arg};
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let input = get_content(parse_input()).await;
    deserialize(input.unwrap()).await;

    Ok(())
}

async fn get_content(url: String) -> Result<String, Error> {
    let body = reqwest::get(&url).await?.text().await?;
    Ok(body)
}

fn parse_input() -> String {
    let matches = App::new("Rosten")
        .version("0.1.1")
        .author("Stian Eklund. <stian.eklund@gmail.com>")
        .about("Get shipment status of your Bring & Posten packages")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
            Arg::with_name("track")
                .short("t")
                .long("track")
                .help("Get package status")
                .takes_value(true),
        )
        .get_matches();

    let input = matches.value_of("track").unwrap();
    String::from(format!(
        "https://tracking.bring.com/api/v2/tracking.json?q={}",
        input
    ))
}

async fn deserialize(buf: String) {
    let deserialized: Result<BringResponse, serde_json::Error> = serde_json::from_str(buf.trim());

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
                        } => println!(
                            "Product Name: {}\nPackage number: {}",
                            product_name, package_number
                        ),
                        _ => println!("Not covered"),
                    }
                    for n in 0..package_set.event_set.len() {
                        match package_set.event_set[n] {
                            Eventset {
                                description: Some(ref description),
                                status: Some(ref status),
                                ..
                            } => println!("Description: {}\nStatus: {}", description, status),
                            _ => println!("Not covered"),
                        }
                    }
                }
            }
        }
        Err(_) => deserialize_err(&buf).await,
    }
}

async fn deserialize_err(buf: &String) {
    let deserialized: Result<ErrorConsignmentSet, serde_json::Error> = serde_json::from_str(&buf);
    match deserialized {
        Ok(deserialized) => {
            eprintln!(
                "Error: {}, Code:{}",
                deserialized.error.message, deserialized.error.code
            );
        }
        Err(e) => eprintln!(
            "Error while deserializing, please check if your tracking number is valid. {}",
            e
        ),
    }
}
