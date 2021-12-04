//extern crate base64;
use reqwest::Result;
use serde_json::Value;
pub fn ascii_to_char(code: u8) -> char {
    std::char::from_u32(code as u32).unwrap_or('_')
}
fn ascii_to_string(code: Vec<u8>) -> Vec<String> {
    let mut test: Vec<String> = vec![];
    let num = code.len();
    let mut count = 0;
    while count < num {
        let mut url = String::new();
        while code[count] != 13 && code[count] != 10 {
            url.push(ascii_to_char(code[count]));
            count += 1;
        }
        test.push(url);
        if code[count] == 10 {
            count += 1;
        } else {
            count += 2;
        }
    }
    test
}

fn ascii_to_string2(code: Vec<u8>) -> String {
    let mut test: String = String::new();
    for cor in code.into_iter() {
        test.push(ascii_to_char(cor));
    }
    test
}
pub fn get_the_key(paths: Vec<String>) -> Result<Vec<Vec<String>>> {
    let mut output: Vec<Vec<String>> = vec![];
    for apath in paths {
        let temp = reqwest::blocking::get(apath)?.bytes()?.to_vec();
        let code = base64::decode(temp);
        match code {
            Ok(input) => {
                output.push(ascii_to_string(input));
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
    (&input[1..length - 1]).to_string()
}
enum Tcp {
    Ss,
    V2,
}
#[derive(Clone)]
pub struct Information {
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
impl Information {
    pub fn information_to_list(&self) -> Vec<String> {
        let mut output = vec![];
        output.push(format!("link: {}", self.get_the_link()));
        output.push(format!("func: {}", self.func));
        output.push(format!("urls: {}", self.urls));
        output.push(format!("add: {}", self.add));
        output.push(format!("port: {}", self.port));
        output.push(format!("name: {}", self.ps));
        output
    }
    pub fn running_json(&self) -> String {
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
            self.func, self.add, remove_quotation(self.port.clone()), remove_quotation(self.aid.clone()), self.id, self.path, self.path, self.net, self.path
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
}}",self.add,self.net,self.id,remove_quotation(self.port.clone()))
        }

    }
    fn get_the_link(&self) -> String {
        let mut temp = String::new();
        if self.func == *"\"vmess\"" {
            temp.push_str(&format!(
                "vmess://{}:{}-{}@{}:{}/#{}",
                &remove_quotation(self.net.clone()),
                &remove_quotation(self.id.clone()),
                &remove_quotation(self.aid.clone()),
                &remove_quotation(self.add.clone()),
                &remove_quotation(self.port.clone()),
                &remove_quotation(self.ps.clone())
            ))
        } else {
            temp = self.urls.clone();
        }
        temp
    }
    pub fn get_the_json_node(&self) -> String {
        format!(
            "{{
    \"func\":{},
    \"url\":\"{}\",
    \"add\":{},
    \"aid\":{},
    \"host\":{},
    \"id\":{},
    \"net\":{},
    \"path\":{},
    \"port\":{},
    \"ps\":{},
    \"tls\":{},
    \"type\":{}
}},\n",
            self.func,
            self.urls,
            self.add,
            self.aid,
            self.host,
            self.id,
            self.net,
            self.path,
            self.port,
            self.ps,
            self.tls,
            self.typpe
        )
    }
    pub fn new(url: String) -> Self {
        let aurl: Vec<char> = url.chars().collect();
        let url_type = {
            if aurl[0] == 's' {
                Tcp::Ss
            } else {
                Tcp::V2
            }
        };
        match url_type {
            Tcp::Ss => {
                // 预处理，去除ss://
                let newurl = (&url[5..]).to_string();
                // 用@分割字符串
                let first: Vec<&str> = newurl.split('@').collect();
                // 传来的节点补全最后一位解析
                let header = first[0].to_string() + "=";
                // 解析，解析结果会返回一个function和密码，中间通过分号分割
                let header2 = ascii_to_string2(base64::decode(header.as_bytes()).unwrap());
                // 通过分号切开两个内容
                let header3: Vec<&str> = header2.split(':').collect();
                let net = format!("\"{}\"", header3[0]);
                let id = format!("\"{}\"", header3[1]);

                let first_temp = first[1].to_string();
                let second: Vec<&str> = first_temp.split('#').collect();
                let ps0 = urlencoding::decode(second[1]).unwrap();
                let ps = format!("\"{}\"", ps0);

                let second_temp = second[0].to_string();
                let third: Vec<&str> = second_temp.split(':').collect();
                let add = format!("\"{}\"", third[0]);
                let port = format!("\"{}\"", third[1]);
                Information {
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
            }
            Tcp::V2 => {
                let newurl = &url[8..];
                let json = ascii_to_string2(base64::decode(newurl.to_string().as_bytes()).unwrap());
                let v: serde_json::Result<Value> = serde_json::from_str(json.as_str());
                match v {
                    Ok(input) => {
                        Information {
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
                    Err(_) => Information {
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
