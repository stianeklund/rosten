 /// Rosten
/// Track Bring packages in Rust
/// Bring Tracking API
/// For testing use "TESTPACKAGE-AT-PICKUPPOINT" as tracking number
/// https://www.mybring.com/tracking/api

/// Request parameters
/// Query param     Type        Description
/// q               string      Reference, package number, shipment num to query
/// callback        string      Optional JSONP callback function name
///
/// Endpoints for Base URL
///
/// Method      Endpoint                        Usage
/// GET         /tracking{mediaTypeExtension}   Get information about a shipment
///
/// Request string for JSON would look like tracking.bring.com/tracking.json?q=trackingnumber

extern crate curl;
extern crate clap;
extern crate rustc_serialize;

use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use rustc_serialize::json::Json;
use curl::easy::Easy;
use std::fmt::Display;

pub fn main() {

    let path = Path::new("~/projects/rosten/tracking.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(e) => panic!("unable to open {}: {}", display, e.description()),
        Ok(file) => file,
    };
    // #[derive(RustcDecodable, RustcEncodable)]
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let data = Json::from_str(&buffer).unwrap_or_else(|e| {
        panic!("Failed to parse JSON, error: {}", e);
    });
    // println!("data: {}", data);
    let obj = data.as_object().unwrap();
    // let foo = obj.get("foo");

    // println!("array? {:?}", foo.as_array());
    // println!("u64? {:?}", foo.as_u64());
    // println!("data: {}", data);
    let mut parsed_data = String::new();
    for (key, value) in obj.iter() {
        // println!("{}: {}", key, match *value {
        let parsed_data = match *value {
            Json::String(ref v) => format!("{} (string)", v),
            Json::Array(ref v) => format!("{:?} (array)", v),
            Json::Object(ref v) => format!("{:?} (object)", v),
            Json::U64(v) => format!("{} (u64)", v),
            _ => format!("other match")
        };
    }
    }



    // TODO Implement match on wanted output instead of .unwrap()
    // println!("{}", json.find_path(&["consignmentSet", "packageSet"]).unwrap());
    // println!("{}", json.find_path(&{"consignmentSet", ["PhoneNumbers"] }).unwrap());




// TODO Handle external GET
//    let mut easy = Easy::new();
//    easy.url(&json_file).unwrap();
//    easy.get("TESTPACKAGE-AT-PICKUPPOINT");

// get_http_body from url
//    let http_body = std::str::from_utf8(http_response.get_http_body()).unwrap_or_else(|err| {
//    panic!("Failed to get response {}; error is {}", url, err); // TODO Error handling
//    });








// import io, os, filecmp // assert_eq!()
// import json, requests
// import argparse

// parser = argparse.ArgumentParser()
// parser.add_argument("sid")
// args = parser.parse_args()

// sid = args.sid // tracking ID

// params = dict(
//             q=sid
//         )
//
// resp = requests.get(url=url, params=params)
// data = json.loads(resp.text)
//
// path = '/home/ix/src/posten/data'
// first = True
// f_i = 0
// f_new = f_old = path + '/' + 'posten-{0}-{1}'.format(sid, f_i) + '.json'
// if os.path.exists(f_new):
//     first = False
//     while os.path.exists(f_new):
//             f_old = path + '/' + 'posten-{0}-{1}'.format(sid, f_i) + '.json'
//             f_i += 1
//             f_new = path + '/' + 'posten-{0}-{1}'.format(sid, f_i) + '.json'
// #print 'f_old: ' + f_old
// #print 'f_new: ' + f_new
//
// # print status for i3
// if not os.path.exists('/tmp/posten'):
//     os.mkdir('/tmp/posten')
//
// e_i = 0
// events = []
// for i in range(0, len(data['consignmentSet'][0]['packageSet'][0]['eventSet'])):
//     events.append(data['consignmentSet'][0]['packageSet'][0]['eventSet'][i]['status'])
// e_i = 0
// while events[e_i] == u'NOTIFICATION_SENT':
//     e_i = e_i + 1
//
// with open('/tmp/posten/' + sid, 'w') as status_file:
//     status_file.write(events[e_i])
//
// with open(f_new, 'w') as outfile:
//      json.dump(data, outfile, sort_keys = True, indent = 4, ensure_ascii=False)
//
// if not first:
//     if filecmp.cmp(f_old, f_new):
//         print 'no change'
//         os.remove(f_new)
//     else:
//         print 'new entry'
