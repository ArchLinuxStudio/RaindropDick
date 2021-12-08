use crate::spider::Information;
use serde_json::{json, Value};
use std::{
    env,
    fs::{self, File},
    io::{prelude::*, Result},
    path::Path,
};
pub enum Save {
    Storage,
    Running,
    V2ray,
    Subscribes,
}
fn create_storage_before() {
    let home = env::var("HOME").unwrap();
    fs::create_dir_all(home + "/.config/tv2ray").unwrap();
}
pub fn create_json_file(save: Save, input: String) -> Result<()> {
    let home = env::var("HOME").unwrap();
    let location = match save {
        Save::Storage => format!("{}/.config/tv2ray/storage.json", home),
        Save::Running => format!("{}/.config/tv2ray/running.json", home),
        Save::V2ray => format!("{}/.config/tv2ray/v2core.json", home),
        Save::Subscribes => format!("{}/.config/tv2ray/subscribes.json", home),
    };
    let path = Path::new(location.as_str());
    let mut file = File::create(&path)?;
    //let storge: String = input;
    file.write_all(input.as_bytes())?;
    Ok(())
}
fn get_json(save: Save) -> Result<String> {
    let home = env::var("HOME").unwrap();
    let location = match save {
        Save::Storage => format!("{}/.config/tv2ray/storage.json", home),
        Save::Running => format!("{}/.config/tv2ray/running.json", home),
        Save::V2ray => format!("{}/.config/tv2ray/v2core.json", home),
        Save::Subscribes => format!("{}/.config/tv2ray/subscribes.json", home),
    };
    let mut file = File::open(location)?;
    let mut output = String::new();
    file.read_to_string(&mut output).unwrap();
    Ok(output)
}
pub fn start_v2core() -> String {
    create_storage_before();
    let message = match get_json(Save::V2ray) {
        Ok(output) => output,
        Err(_) => {
            let core = json!({
                "v2core":"/usr/bin/v2ray"
            })
            .to_string();
            create_json_file(Save::V2ray, core.clone()).unwrap_or_else(|err| panic!("{}", err));
            core
        }
    };
    let v: Value = serde_json::from_str(message.as_str()).unwrap();
    let message_pre = v["v2core"].to_string();
    crate::spider::remove_quotation(message_pre)
}
pub fn get_subs() -> Vec<String> {
    create_storage_before();
    let messages = match get_json(Save::Subscribes) {
        Ok(output) => output,
        Err(_) => {
            create_json_file(Save::Storage, "[]".to_string())
                .unwrap_or_else(|err| panic!("{}", err));
            "[]".to_string()
        }
    };
    let mut subscribes = Vec::new();
    let v: Value = serde_json::from_str(messages.as_str()).unwrap();
    let mut index = 0;
    while v[index] != Value::Null {
        let sub = v[index]["url"].to_string();
        let length = sub.len();
        let sub = (&sub[1..length - 1]).to_string();
        subscribes.push(sub);
        index += 1;
    }
    subscribes
}
pub fn start() -> Vec<Vec<Information>> {
    create_storage_before();
    let messages = match get_json(Save::Storage) {
        Ok(output) => output,
        Err(_) => {
            create_json_file(Save::Storage, "[]".to_string())
                .unwrap_or_else(|err| panic!("{}", err));
            "[]".to_string()
        }
    };
    let mut informations = Vec::new();
    let v: Value = serde_json::from_str(messages.as_str()).unwrap();
    let mut index = 0;
    while v[index] != Value::Null {
        let mut index2 = 0;
        let w = v[index].clone();
        let mut information = Vec::new();
        while w[index2] != Value::Null {
            let the_url = w[index2]["url"].to_string();
            let length = the_url.len();
            let instore = &the_url[1..length - 1];
            information.push(Information {
                urls: instore.to_string(),
                func: w[index2]["func"].to_string(),
                add: w[index2]["add"].to_string(),
                aid: w[index2]["aid"].to_string(),
                host: w[index2]["host"].to_string(),
                id: w[index2]["id"].to_string(),
                net: w[index2]["net"].to_string(),
                path: w[index2]["path"].to_string(),
                port: w[index2]["port"].to_string(),
                ps: w[index2]["ps"].to_string(),
                tls: w[index2]["tls"].to_string(),
                typpe: w[index2]["type"].to_string(),
            });
            index2 += 1;
        }
        informations.push(information);
        //let names = v[index]["ps"].to_string();
        //start.add_item(remove_quotation(names), url);
        index += 1;
    }
    informations
}
