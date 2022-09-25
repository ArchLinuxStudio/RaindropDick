//extern crate base64;
use reqwest::Result;
//use serde::{Deserialize, Serialize};
use v2raylinks::*;
pub async fn get_the_links(paths: Vec<String>) -> Result<Vec<Vec<Links>>> {
    let mut output: Vec<Vec<Links>> = vec![];
    for apath in paths {
        let temp = reqwest::get(apath).await?.bytes().await?.to_vec();
        let code = base64::decode(temp);
        match code {
            Ok(input) => {
                output.push(get_links(&String::from_utf8(input).unwrap()));
            }
            Err(_) => {
                return Ok(vec![]);
            }
        }
    }
    Ok(output)
}

pub fn remove_quotation(input: String) -> String {
    let length = input.len();
    (input[1..length - 1]).to_string()
}
