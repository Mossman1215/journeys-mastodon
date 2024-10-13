use std::{collections::HashMap, env, fs::File, io::{BufReader, Read}, ops::Sub, process, str::FromStr, sync::Arc};
use std::time::{Duration, SystemTime};
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, NaiveTime, TimeDelta, TimeZone};
use geojson::{Feature, FeatureCollection, GeoJson};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

struct Auth {
    access_token: String,
}


#[tokio::main]
async fn main() {
    println!("Checking for crashes");
    let mut auth_token = Auth {
        access_token: "".to_string(),
    };
    if env::var("ACCESS_TOKEN").is_ok() {
        //do something
        //read value into config struct
        auth_token = Auth {
            access_token: env::var("ACCESS_TOKEN").unwrap(),
        };
    } else {
        println!("failed to find ACCESS_TOKEN. Exiting...");
        process::exit(1);
    }
    let dt1: DateTime<Local> = Local::now();
    let now = TimeDelta::new(dt1.timestamp(),0).unwrap();
    let resp = reqwest::get("https://www.journeys.nzta.govt.nz/assets/map-data-cache/delays.json")
        .await.unwrap().text().await.expect("failed to fetch nzta data");
    let delays: Value = serde_json::from_str(&resp).expect("parse failed");
    //Duration::new(900, 0);
    //Duration::new(lastUpdated)
    let lastupdate = TimeDelta::new(delays["lastUpdated"].as_i64().unwrap(),0).unwrap();
    let duration = TimeDelta::new(300, 0).unwrap();
    let re = Regex::new(r"SH\s[0-9]+").unwrap();
    //if now sub lastupdated > 300 check for closures with matching properties
    if  now-lastupdate<duration{
        for feature in delays["features"].as_array().unwrap() {
            let desc = feature["properties"].as_object().unwrap()["EventDescription"].as_str().unwrap();
            let last_edit_time = NaiveDateTime::parse_from_str( feature["properties"].as_object().unwrap()["LastEdited"].as_str().unwrap(),"%Y-%m-%d %H:%M:%S").unwrap();
            let last_edit_stamp = TimeDelta::new(last_edit_time.and_local_timezone(FixedOffset::east_opt(12*3600).unwrap()).unwrap().timestamp(),0).unwrap();
            let last_update_stamp = TimeDelta::new(feature["properties"].as_object().unwrap()["lastUpdated"].as_i64().unwrap(), 0).unwrap();
            if desc == "Crash" && now - last_edit_stamp<duration && now - last_update_stamp<duration{
                let m = re.find(feature["properties"].as_object().unwrap()["LocationArea"].as_str().unwrap());
                let mut highway_hash = String::from_str("").unwrap();
                match m {
                    None => println!("no highway type found"),
                    Some(m) => highway_hash = (String::from_str("#").unwrap()+m.as_str()).replace(" ", ""),
                }
                let message = format!("{}\n{}\nLast Updated: {}\n{}",
                    feature["properties"].as_object().unwrap()["Name"].as_str().unwrap(),
                    feature["properties"].as_object().unwrap()["EventComments"].as_str().unwrap(),
                    feature["properties"].as_object().unwrap()["LastUpdatedNice"].as_str().unwrap(),
                    highway_hash
                );
                let mut map = HashMap::new();
                println!("sending message to mastodon");
                map.insert("status", message.as_str());
                map.insert("visibility", "public");

                let client = reqwest::Client::new();
                let res2 = client.post("https://g2s.mountainmoss.nz/api/v1/statuses")
                    .bearer_auth(auth_token.access_token.clone())
                    .header("User-Agent", "journeys-mastodon")
                    .json(&map)
                    .send()
                    .await.unwrap();
                if res2.status() != 200 {
                    println!("error happened: {},{}",res2.status(),res2.text().await.unwrap())
                }
            }
        }
    }
}

#[test]
fn property_parse() {
    // Open the file in read-only mode with buffer.
    let mut file = File::open("fixtures/delays.json").expect("should pass");

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let delays: Value = serde_json::from_str(&data).expect("parse failed");
    assert!(delays.is_object());
    assert!(delays["features"].is_array());
    assert_eq!(delays["features"].as_array().unwrap().len(),143);
    let test_event = delays["features"].as_array().unwrap()[0].clone();
    assert!((test_event["properties"].as_object().unwrap())["Name"].as_str() == Some("Road Closure: SH 1 Onewa Rd On-ramp Southbound"));
    assert!(test_event["properties"].as_object().unwrap()["LastEdited"].as_str() == Some("2024-06-02 20:03:35"));
    assert!(delays["lastUpdated"].as_i64() == Some(1717315415));
}

#[test]
fn regex_capture() {
    let mut file = File::open("fixtures/delays.json").expect("should pass");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let geojson = data.as_str().parse::<GeoJson>().unwrap();
    let delays = FeatureCollection::try_from(geojson).unwrap();
    let test_event = delays.features[4].clone();
    assert_eq!(test_event.property("LocationArea").unwrap(),"SH 2 Kaitoke to Featherston (Remutaka Hill)");
    assert_eq!(test_event.property("EventDescription").unwrap(),"Crash");
    let re = Regex::new(r"SH\s[0-9]+").unwrap();
    let m = re.find(test_event.property("LocationArea").unwrap().as_str().unwrap());
    let mut highway_hash = "";
    match m {
        None => println!("no highway type found"),
        Some(m) => highway_hash = m.as_str(),
    }
    assert_eq!(highway_hash,"SH 2")
}

#[test]
fn parse_regions() {
    let mut file = File::open("Data/NZ_Regions_LocalGovt.geojson").expect("should pass");
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let geojson = data.as_str().parse::<GeoJson>().unwrap();
    let regions = FeatureCollection::try_from(geojson).unwrap();
    assert_eq!(regions.features.len(),17);
    assert_eq!(regions.features[8].property("REGC2023_1").unwrap(),"Wellington Region")
}
#[test]
fn property_parse2() {
    // Open the file in read-only mode with buffer.
    let mut file = File::open("fixtures/delays2.json").expect("should pass");

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let delays: Value = serde_json::from_str(&data).expect("parse failed");
    assert!(delays.is_object());
    assert!(delays["features"].is_array());
    assert_eq!(delays["features"].as_array().unwrap().len(),166);
    let test_event = delays["features"].as_array().unwrap()[0].clone();
    assert!((test_event["properties"].as_object().unwrap())["Name"].as_str() == Some("Road Closure: SH 1 Oteha Valley to Silverdale, Northbound"));
    assert!(test_event["properties"].as_object().unwrap()["LastEdited"].as_str() == Some("2024-10-13 16:50:41"));
    assert!(test_event["properties"].as_object().unwrap()["lastUpdated"].as_i64() == Some(1728791441));
    assert!(delays["lastUpdated"].as_i64() == Some(1728793892));
}