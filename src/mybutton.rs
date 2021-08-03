use crate::spider::ascii_to_char;
use cursive::views::Button;
use cursive::views::Dialog;
use cursive::views::LinearLayout;
use cursive::views::SelectView;
use cursive::views::TextView;
use cursive::Cursive;
use serde_json::Result;
use serde_json::Value;
use std::process::Command;
use std::{env, fs::File, io::prelude::*, path::Path};
extern crate base64;

enum Tcp {
    Ss,
    V2,
}
fn ascii_to_string(code: Vec<u8>) -> String {
    let mut test: String = String::new();
    for cor in code.into_iter() {
        test.push(ascii_to_char(cor));
    }
    test
}
#[derive(Clone)]
pub struct MyButton {
    pub func: String,
    pub urls: String,
    pub add: String,
    pub aid: String,
    pub host: String,
    pub id: String,
    pub net: String,
    pub path: String,
    pub port: String,
    pub ps: String,
    pub tls: String,
    pub typpe: String,
}

impl MyButton {
     fn get_the_json(&self) -> String {
        let temp = self.port.clone();
        let length = temp.len();
        let port: String = (&temp[1..length - 1]).to_string();
        let temp2 = self.aid.clone();
        let length2 = temp2.len();
        let aid: String = (&temp2[1..length2 - 1]).to_string();
        if self.func == *"\"vmess\"" {
        format!(
            "{{
    \"inbounds\":[{{
        \"port\":8889,
        \"listen\":\"127.0.0.1\",
        \"protocol\":\"http\",
        \"settings\":{{
            \"udp\": true
        }}
    }}],
    \"outbounds\":[{{
        \"protocol\":{},
        \"sendThrough\": \"0.0.0.0\",
        \"settings\":{{
            \"vnext\": [{{
                \"address\": {},
                \"port\":{},
                \"users\":[{{
                    \"alterId\": {},
                    \"id\":{}
                }}]
            }}]
        }},
        \"streamSettings\":{{
            \"dsSettings\": {{
                \"path\": {}
            }},
            \"httpSettings\":{{
                \"host\": [
                ],
                \"path\":{}
            }},
            \"kcpSettings\": {{
                \"congestion\": false,
                \"downlinkCapacity\":20,
                \"header\": {{
                    \"type\": \"none\"
                }},
                \"mtu\": 1350,
                \"readBufferSize\": 1,
                \"tti\": 20,
                \"uplinkCapacity\": 5,
                \"writeBufferSize\": 1
            }},
            \"network\": {},
            \"quicSettings\":{{
                \"header\": {{
                    \"type\":\"none\"
                }},
                \"key\": \"\",
                \"security\":\"\"
            }},
            \"security\":\"none\",
            \"sockopt\":{{
                \"mark\": 255,
                \"tcpFastOpen\": false,
                \"tproxy\": \"off\"
            }},
            \"tcpSettings\": {{
                \"header\": {{
                    \"request\" :{{
                        \"headers\":{{
                        }},
                        \"method\": \"GET\",
                        \"path\":[
                        ],
                        \"version\":\"1.1\"
                    }},
                    \"type\": \"none\"
                }}
            }},
            \"tlsSettings\": {{
                \"allowInsecure\": true,
                \"allowInsecureCiphers\": true,
                \"alpn\":[
                ],
                \"certificates\":[
                ],
                \"disableSessionResumption\":true,
                \"disableSystemRoot\":true,
                \"serveName\": \"\"
            }},
            \"wsSettings\" :{{
                \"headers\" :{{
                }},
                \"path\":{}
            }},
            \"xtlsSettings\":{{
                \"allowInsecure\":true,
                \"allowInsecureCiphers\":true,
                \"alpn\":[
                ],
                \"certificates\":[
                ],
                \"disableSessionResumption\": false,
                \"disableSystemRoot\": true,
                \"serveName\":\"\"
            }},
            \"tag\":\"outBound_PROXY\"
        }}
    }},
    {{
        \"protocol\":\"freedom\",
        \"tag\": \"direct\",
        \"settings\":{{}}
    }}],
    \"routing\": {{
        \"domainStrategy\": \"IPOnDemand\",
        \"rules\":[{{
            \"type\":\"field\",
            \"ip\":[\"geoip:private\"],
            \"outboundTag\": \"direct\"
        }}]
    }}
}}",
            self.func, self.add, port, aid, self.id, self.path, self.path, self.net, self.path
        )
        }else{
            format!("{{
    \"api\":{{
        \"service\":[
            \"HandlerService\",
            \"LoggerService\",
            \"StatsService\"
            ],
        \"tag\": \"_QV2RAY_API_\"
    }},
    \"dns\":{{
        \"service\":[
            \"1.1.1.1\",
            \"8.8.8.8\",
            \"8.8.4.4\"
        ]
    }},
    \"inbounds\":[
        {{
            \"listen\":\"127.0.0.1\",
            \"protocol\": \"dokodemo-door\",
            \"port\": 15490,
            \"settings\":{{
                \"address\":\"127.0.0.1\"
            }},
            \"sniffing\":{{
            }}
        }},
        {{
            \"listen\":\"127.0.0.1\",
            \"port\": 8889,
            \"protocol\":\"http\",
            \"settings\":{{
                \"allowTransparent\":true,
                \"timeout\": 300,
                \"userLevel\":0
            }},
            \"sniffing\":{{
                \"enabled\":false
            }},
            \"tag\":\"http_IN\"
        }},
        {{
            \"listen\": \"127.0.0.1\",
            \"port\": 1089,
            \"protocol\": \"socks\",
            \"settings\" :{{
                \"auth\": \"noauth\",
                \"ip\": \"127.0.0.1\",
                \"udp\": true,
                \"userLevel\": 0
            }},
            \"sniffing\":{{
                \"enabled\":false
            }},
            \"tag\": \"socks_IN\"
        }},
        {{
            \"listen\": \"127.0.0.1\",
            \"port\": 12345,
            \"protocol\" : \"dokodemo-door\",
            \"settings\":{{
                \"address\":\"\",
                \"followRediect\": true,
                \"network\": \"tcp\",
                \"port\":0,
                \"timeout\":0,
                \"userLevel\":0
            }},
            \"sniffing\":{{
                \"destOverride\":[
                    \"http\",
                    \"tls\"
                ],
                \"enabled\": true
            }},
            \"streamSettings\":{{
                \"sockopt\":{{
                    \"tproxy\": \"tproxy\"
                }}
            }},
            \"tag\": \"tproxy_IN\"
        }},
        {{
            \"listen\": \"::1\",
            \"port\": 12345,
            \"protocol\": \"dokodemo-door\",
            \"settings\": {{
                \"address\": \"\",
                \"followRediect\": true,
                \"network\": \"tcp\",
                \"port\": 0,
                \"timeout\": 0,
                \"userLevel\": 0
            }},
            \"sniffing\": {{
                \"destOverride\": [
                    \"http\",
                    \"tls\"
                ],
                \"enabled\": true
            }},
            \"streamSettings\": {{
                \"sockopt\": {{
                    \"tproxy\": \"tproxy\"
                }}
            }},
            \"tag\": \"tproxy_IN_V6\"
        }}
    ],
    \"log\": {{
        \"loglevel\": \"warning\"
    }},
    \"outbounds\":[
        {{
            \"protocol\": \"shadowsocks\",
            \"sendThrough\": \"0.0.0.0\",
            \"settings\": {{
                \"servers\" :[
                    {{
                        \"address\":{},
                        \"email\": \"\",
                        \"level\": 0,
                        \"method\": {},
                        \"ota\":false,
                        \"password\":{},
                        \"port\":{}
                    }}
                ]
            }},
            \"streamSettings\": {{
                \"sockopt\":{{
                    \"mark\": 255
                }}
            }},
            \"tag\": \"outBound_PROXY\"
        }},
        {{
            \"protocol\": \"freedom\",
            \"sendThrough\": \"0.0.0.0\",
            \"settings\": {{
                \"domainStrategy\": \"AsIs\",
                \"redirect\": \":0\",
                \"userLevel\": 0
            }},
            \"streamSettings\": {{
                \"sockopt\": {{
                    \"mark\": 255
                }}
            }},
            \"tag\": \"outBound_DIRECT\"
        }},
        {{
            \"protocol\": \"blackhole\",
            \"sendThrough\": \"0.0.0.0\",
            \"settings\": {{
                \"response\": {{
                    \"type\": \"none\"
                }}
            }},
            \"streamSettings\": {{
                \"sockopt\": {{
                    \"mark\": 255
                }}
            }},
            \"tag\": \"outBound_BLACKHOLE\"
        }}
    ],
    \"policy\": {{
        \"system\": {{
            \"statsInboundDownlink\": true,
            \"statsInboundUplink\": true,
            \"statsOutboundDownlink\": true,
            \"statsOutboundUplink\": true
        }}
    }},
    \"routing\": {{
        \"domainStrategy\": \"AsIs\",
        \"rules\": [
            {{
                \"inboundTag\": [
                    \"_QV2RAY_API_INBOUND_\"
                ],
                \"outboundTag\": \"_QV2RAY_API_\",
                \"type\": \"field\"
            }},
            {{
                \"ip\": [
                    \"geoip:private\"
                ],
                \"outboundTag\": \"outBound_DIRECT\",
                \"type\": \"field\"
            }},
            {{
                \"ip\": [
                    \"geoip:cn\"
                ],
                \"outboundTag\": \"outBound_DIRECT\",
                \"type\": \"field\"
            }},
            {{
                \"domain\": [
                    \"geosite:cn\"
                ],
                \"outboundTag\": \"outBound_DIRECT\",
                \"type\": \"field\"
            }}
        ]
    }},
    \"stats\": {{
    }}
}}",self.add,self.net,self.id,port)
        }
    }

    pub fn output(&self) -> Dialog {
        fn running_json(s: &mut Cursive, name: &MyButton) {
            let mut json = String::new();
            //let temp = name.port.clone();
            //let length = temp.len();
            //let port: String = (&temp[1..length - 1]).to_string();
            //let temp2 = name.aid.clone();
            //let length2 = temp2.len();
            //let aid: String = (&temp2[1..length2 - 1]).to_string();
            let output = name.get_the_json();
            json.push_str(output.as_str());
            let home = env::var("HOME").unwrap();
            let location = home + "/.config/tv2ray/running.json";
            let path2 = Path::new(location.as_str());
            //let display = path.display();
            //let path2 = Path::new("storage.json");
            let display2 = path2.display();
            let mut file2 = match File::create(&path2) {
                Err(why) => panic!("couldn't create {}: {}", display2, why.to_string()),
                Ok(file2) => file2,
            };


            if let Err(why) = file2.write_all(json.as_bytes()) {
                panic!("couldn't write to {}: {}", display2, why.to_string())
            }
            Command::new("pkill")
                .arg("v2ray")
                .output()
                .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

            let home2 = env::var("HOME").unwrap();
            let location = home2.clone() + "/.config/tv2ray/v2core.json";
            let path = Path::new(location.as_str());
            //let display = path.display();
            let mut file = match File::open(&path) {
                // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
                Err(_) => {
                    let path2 = Path::new(location.as_str());
                    let display2 = path2.display();
                    let mut file2 = match File::create(&path2) {
                        Err(why) => panic!("couldn't create {}: {}", display2, why.to_string()),
                        Ok(file2) => file2,
                    };
                    let mut storge2: String = String::new();
                    storge2.push_str("{\n\"v2core\":\"/usr/v2ray\"\n}");
                    // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
                    if let Err(why) = file2.write_all(storge2.as_bytes()) {
                        panic!("couldn't write to {}: {}", display2, why.to_string())
                    }
                    let path3 = Path::new(location.as_str());
                    File::open(&path3).unwrap()
                }
                Ok(file) => file,
            };
            let mut ss = String::new();
            let mut content: String = String::new();
            match file.read_to_string(&mut ss) {
                Err(_) => {}
                Ok(_) => {
                    let v: Value = serde_json::from_str(ss.as_str()).unwrap();
                    let temp = v["v2core"].to_string();
                    let length = temp.len();
                    content = (&temp[1..length - 1]).to_string();
                }
            }

            Command::new("nohup")
                .arg(content)
                .arg("-config")
                .arg(home2.clone() + "/.config/tv2ray/running.json")
                .arg(">")
                .arg(home2 + "/.config/tv2ray/test.log")
                .arg("2>&1")
                .arg("&")
                .spawn()
                .expect("failed");

            s.pop_layer();
        }
        let mut select = SelectView::<MyButton>::new().on_submit(running_json);
        select.add_item("<start>", self.clone());
        Dialog::around(
            LinearLayout::horizontal()
                .child(TextView::new(format!(
                    "Name:{}\nUrl:{}\nport:{}\nfunction:{}\ncompany:{}",
                    self.ps, self.urls, self.port, self.func, self.add
                )))
                .child(
                    LinearLayout::vertical()
                        .child(select)
                        .child(Button::new("quit", |s| {
                            s.pop_layer();
                        })),
                ),
        )
    }
    pub fn new(url: String) -> MyButton {
        let mut test: Tcp = Tcp::V2;
        for pair in url.chars() {
            if pair == 's' {
                test = Tcp::Ss;
                break;
            }
            if pair == 'v' {
                test = Tcp::V2;
                break;
            }
        }
        match test {
            Tcp::Ss =>{
                                // 预处理，去除ss://
                let newurl = (&url[5..]).to_string();
                // 用@分割字符串
                let first: Vec<&str> = newurl.split('@').collect();
                // 传来的节点补全最后一位解析
                let header = first[0].to_string() + "=";
                // 解析，解析结果会返回一个function和密码，中间通过分号分割
                let header2 = ascii_to_string(base64::decode(header.as_bytes()).unwrap());
                // 通过分号切开两个内容
                let header3: Vec<&str> = header2.split(':').collect();
                let net = format!("\"{}\"", header3[0].to_string());
                let id = format!("\"{}\"", header3[1].to_string());

                let first_temp = first[1].to_string();
                let second: Vec<&str> = first_temp.split('#').collect();
                let ps0 = urlencoding::decode(second[1]).unwrap();
                let ps = format!("\"{}\"", ps0.to_string());

                let second_temp = second[0].to_string();
                let third: Vec<&str> = second_temp.split(':').collect();
                let add = format!("\"{}\"", third[0].to_string());
                let port = format!("\"{}\"", third[1].to_string());
                MyButton {
                    urls: url,
                    func: "\"ss\"".to_string(),
                    add,
                    aid: "\"unknown\"".to_string(),
                    host: "\"\"".to_string(),
                    id,
                    net,
                    path: "\"unknown\"".to_string(),
                    port,
                    ps,
                    tls: "\"unknown\"".to_string(),
                    typpe: "\"unknown\"".to_string(),

                }
            },
            Tcp::V2 => {
                let newurl = &url[8..];
                let json = ascii_to_string(base64::decode(newurl.to_string().as_bytes()).unwrap());
                let v: Result<Value> = serde_json::from_str(json.as_str());
                match v {
                    Ok(input) => {
                        MyButton {
                            //company : input["add"].to_string(),
                            urls: url,
                            func: "\"vmess\"".to_string(),
                            add: input["add"].to_string(),
                            aid: input["aid"].to_string(),
                            host: input["host"].to_string(),
                            id: input["id"].to_string(),
                            net: input["net"].to_string(),
                            path: input["path"].to_string(),
                            port: input["port"].to_string(),
                            ps: input["ps"].to_string(),
                            tls: input["tls"].to_string(),
                            typpe: input["type"].to_string(),
                        }
                    }
                    Err(_) => MyButton {
                        urls: url,
                        func: "\"vmess\"".to_string(),
                        add: "\"unknown\"".to_string(),
                        aid: "\"unknown\"".to_string(),
                        host: "\"unknown\"".to_string(),
                        id: "\"unknown\"".to_string(),
                        net: "\"unknown\"".to_string(),
                        path: "\"unknown\"".to_string(),
                        port: "\"unknown\"".to_string(),
                        ps: "\"unknown\"".to_string(),
                        tls: "\"unknown\"".to_string(),
                        typpe: "\"unknown\"".to_string(),
                    },
                }
            }
        }
    }
}
