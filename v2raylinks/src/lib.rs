mod linkregex;
#[cfg(test)]
mod tests;

use linkregex::*;
use serde::{Deserialize, Serialize};

pub trait V2rayRun {
    fn urldecode(&self) -> String;
    fn infomation(&self) -> Vec<String>;
}

#[derive(Deserialize, Debug, Serialize)]
pub enum Links {
    VMESS(Vmess),
    SS(SS),
}

impl PartialEq for Links {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Links::SS(mime), Links::SS(other)) => mime == other,
            (Links::VMESS(mime), Links::VMESS(other)) => mime == other,
            (_, _) => false,
        }
    }
}

impl Links {
    pub fn new(link: &str) -> Self {
        if VMESSLINK.is_match(link) {
            let message = VMESSLINK.captures(link).unwrap().get(1).unwrap().as_str();
            let message = String::from_utf8(base64::decode(message.as_bytes()).unwrap()).unwrap();
            match serde_json::from_str(&message) {
                Ok(jsons) => Links::VMESS(jsons),
                Err(_) => panic!("{}", message),
            }
        } else {
            // then I think it should be ss
            let catptures = SSLINK_WITHINFO.captures(link).unwrap();
            let methodandpassward = &catptures["MethodAndPassward"];
            let methodandpassward =
                String::from_utf8(base64::decode(methodandpassward.as_bytes()).unwrap()).unwrap();
            let methodwithpasswardcap = SSMETHODANDPASSWARD.captures(&methodandpassward).unwrap();
            let method = methodwithpasswardcap["Method"].to_string();
            let passward = methodwithpasswardcap["Passward"].to_string();
            let ip = catptures["Ip"].to_string();
            let port = catptures["Port"].to_string();
            let tips = &catptures["Tips"];
            let tipsfront: Vec<u8> = UTF8UNIT
                .captures_iter(tips)
                .map(|unit| u8::from_str_radix(unit.get(1).unwrap().as_str(), 16).unwrap())
                .collect();

            let tipsend = &tips[3 * tipsfront.len()..];
            let mut linkname = String::from_utf8(tipsfront).unwrap();
            linkname.push_str(tipsend);
            Links::SS(SS {
                method,
                passward,
                ip,
                port,
                linkname,
            })
        }
    }
}

#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub struct Vmess {
    pub v: String,
    pub ps: String,
    pub add: String,
    pub port: String,
    pub id: String,
    pub aid: String,
    pub net: String,
    #[serde(rename = "type")]
    pub vmtype: String,
    pub host: String,
    pub path: String,
    pub tls: String,
    pub sni: Option<String>,
}
#[derive(Clone, Deserialize, Debug, PartialEq, Serialize)]
pub struct SS {
    pub method: String,
    pub passward: String,
    pub ip: String,
    pub port: String,
    pub linkname: String,
}
impl V2rayRun for Vmess {
    fn urldecode(&self) -> String {
        format!(
            "vmess://{}:{}-{}@{}:{}/#{}",
            self.net, self.id, self.aid, self.add, self.port, self.ps
        )
    }
    fn infomation(&self) -> Vec<String> {
        vec![
            "VMESS".to_string(),
            format!("url:  {}", self.urldecode()),
            format!("ps:   {}", self.ps),
            format!("add:  {}", self.add),
            format!("port: {}", self.port),
            format!("id:   {}", self.id),
            format!("aid:  {}", self.aid),
            format!("net:  {}", self.net),
            format!("type: {}", self.vmtype),
            format!("host: {}", self.host),
            format!("path: {}", self.path),
            format!("tls:  {}", self.tls),
            format!(
                "sni:  {}",
                match &self.sni {
                    Some(url) => url,
                    None => "None",
                }
            ),
        ]
    }
}
impl V2rayRun for SS {
    fn urldecode(&self) -> String {
        format!(
            "ss://{}:{}@{}:{}#{}",
            self.method, self.passward, self.ip, self.port, self.linkname
        )
    }
    fn infomation(&self) -> Vec<String> {
        vec![
            "SS".to_string(),
            format!("url:      {}", self.urldecode()),
            format!("linkname: {}", self.linkname),
            format!("method:   {}", self.method),
            format!("passward: {}", self.passward),
            format!("ip:       {}", self.ip),
            format!("port:     {}", self.port),
        ]
    }
}
