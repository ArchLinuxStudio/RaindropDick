use crate::spider::Information;
use serde_json::Value;
use std::{
    env,
    fs::{self, File},
    io::prelude::*,
    io::Error,
    path::Path,
    result::Result,
};
fn create_storage_before() {
    let home = env::var("HOME").unwrap();
    fs::create_dir_all(home + "/.config/tv2ray").unwrap();
}
pub fn create_json_file(input: String) -> Result<(), Error> {
    let home = env::var("HOME").unwrap();
    let location = format!("{}/.config/tv2ray/storage.json", home);
    let path = Path::new(location.as_str());
    let mut file = File::create(&path)?;
    //let storge: String = input;
    file.write_all(input.as_bytes())?;
    Ok(())
}
fn get_json() -> Result<String, Error> {
    let home = env::var("HOME").unwrap();
    let location = format!("{}/.config/tv2ray/storage.json", home);
    let mut file = File::open(location)?;
    let mut output = String::new();
    file.read_to_string(&mut output).unwrap();
    Ok(output)
}
pub fn start() -> Vec<Information> {
    create_storage_before();
    let messages = match get_json() {
        Ok(output) => output,
        Err(_) => {
            if let Err(err) = create_json_file("[]".to_string()) {
                panic!("{}", err);
            };
            "[]".to_string()
        }
    };
    let mut informations = Vec::new();
    let v: Value = serde_json::from_str(messages.as_str()).unwrap();
    let mut index = 0;
    while v[index] != Value::Null {
        let the_url = v[index]["url"].to_string();
        let lenghth = the_url.len();
        let instore = &the_url[1..lenghth - 1];
        informations.push(Information {
            urls: instore.to_string(),
            func: v[index]["func"].to_string(),
            add: v[index]["add"].to_string(),
            aid: v[index]["aid"].to_string(),
            host: v[index]["host"].to_string(),
            id: v[index]["id"].to_string(),
            net: v[index]["net"].to_string(),
            path: v[index]["path"].to_string(),
            port: v[index]["port"].to_string(),
            ps: v[index]["ps"].to_string(),
            tls: v[index]["tls"].to_string(),
            typpe: v[index]["type"].to_string(),
        });
        //let names = v[index]["ps"].to_string();
        //start.add_item(remove_quotation(names), url);
        index += 1;
    }
    informations
}
