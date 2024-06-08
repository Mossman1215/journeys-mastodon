use std::{collections::HashMap, env, fs::File, io::{BufReader, Read}, process, str::FromStr, sync::Arc};

use serde::{Deserialize, Serialize};
use serde_json::Value;

struct Auth {
    access_token: String,
}


#[tokio::main]
async fn main() {
    println!("Hello, world!");
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

    let resp = reqwest::get("https://httpbin.org/ip")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await;
    println!("{resp:#?}");
}

#[test]
fn property_parse() {
        // Open the file in read-only mode with buffer.
        let mut file = File::open("fixtures/delays.json").expect("should pass");
    
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let poo: Value = serde_json::from_str(&data).expect("parse failed");
        assert!(poo.is_object());
        assert!(poo["features"].is_array());
        assert_eq!(poo["features"].as_array().unwrap().len(),143);
        let blerp = poo["features"].as_array().unwrap()[0].clone();
        assert!((blerp["properties"].as_object().unwrap())["Name"].as_str() == Some("Road Closure: SH 1 Onewa Rd On-ramp Southbound"));
        assert!(blerp["properties"].as_object().unwrap()["LastEdited"].as_str() == Some("2024-06-02 20:03:35"));
        assert!(poo["lastUpdated"].as_i64() == Some(1717315415));
}