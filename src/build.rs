extern crate dotenv;
//#[macro_use] extern crate serde_derive;
extern crate serde;
use std::io::prelude::*;
use std::fs::File;
/*
extern crate serde_json;
use serde_json::Map;
use serde_json::Value;
*/
//use std::collections::HashMap;
extern crate indexmap;
use indexmap::IndexMap;
extern crate serde_yaml;

// VariantMap is a strong, independent type who don't need no simplification
type VariantMap = IndexMap<String, IndexMap<String, Vec<String>>>;

fn main() {
    match dotenv::dotenv() {
        Err(e) => println!("Error reading .env file: {}", e),
        _ => (),
    }
    let filename = std::env::var("VARIANT_MAP_FILEPATH")
        .expect("VARIANT_MAP_FILEPATH must be set (check `.env`).");
    let mut fh = File::open(filename.as_str())
        .expect(format!("Error while opening `{}`", filename).as_str());
    /*
    // The json object is wrapped with '{ map: ... } to have it play nice with serde
    let mut variant_map_json = String::new(); //"{ map:".to_string();
    fh.read_to_string(&mut variant_map_json)
        .expect(format!("Error while reading `{}`", filename).as_str());
    ////variant_map_json.push('}');
    //let variant_map: HashMap<String, HashMap<String, Vec<String>>> = serde_json::from_str(variant_map_json.as_str()).expect("Bad JSON");
    //panic!("{:?}", variant_map["rainbow"]);
    let variant_map: Value = serde_json::from_str(variant_map_json.as_str())
        .expect(format!("Error parsing json in `{}`", filename).as_str());
    let variant_map: Map<String, Value> = serde_json::from_value(variant_map).expect("Bad JSON");
    panic!("{:?}", variant_map);
    for (variant_name, variant_obj) in variant_map.iter_mut() {
        let variant_obj: Map<String, Value> = serde_json::from_value(variant_obj).expect("Bad JSON");
        for (suit_name, colors) in variant_obj.iter() {
            let colors = colors.as_array().unwrap();
        }
    }
    */
    let mut variant_map_yaml = String::new(); //"{ map:".to_string();
    fh.read_to_string(&mut variant_map_yaml)
        .expect(format!("Error while reading `{}`", filename).as_str());
    let variant_map: VariantMap = serde_yaml::from_str(variant_map_yaml.as_str()).expect("Bad yaml");
    panic!("{:?}", variant_map["rainbow"]);
}
