use cursive::views::Dialog;
use spider::ascii_to_char;
use serde_json::Value;
use serde_json::Result;
extern crate base64;

enum Tcp {
    Ss,
    V2,
}
fn ascii_to_string(code:Vec<u8>) -> String{
    let mut test:String = String::new();
    for cor in code.into_iter(){
        test.push(ascii_to_char(cor));
    }
    test
}
#[derive(Clone)]
pub struct MyButton {
    //pub urls : String,
    //pub name : String,
    //pub port :String,
    pub func :String,
    //pub company:String,
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
    pub typpe: String

}

impl MyButton{
    pub fn output(&self) -> Dialog {
        Dialog::text(format!("Name:{}\nUrl:{}\nport:{}\nfunction:{}\ncompany:{}", self.ps,self.urls,self.port,self.func,self.add))
                .title(format!("{}", self.add))
                //.button("Quit", Cursive::quit)
                .button("quit", |s|{
                    s.pop_layer();
                })
    }
    pub fn new (url:String) -> MyButton{
        let mut test : Tcp = Tcp::V2; 
        for pair in url.chars(){
            if pair=='s'{
                test =  Tcp::Ss;
                break;
            }
            if pair=='v'{
                test = Tcp::V2;
                break;

            }
        }
        match test {
            Tcp::Ss => {
                return MyButton{
                    urls  : url,
                    func  : "\"ss\"".to_string(),
                    add   : "\"unknown\"".to_string(),
                    aid   : "\"unknown\"".to_string(),
                    host  : "\"unknown\"".to_string(),
                    id    : "\"unknown\"".to_string(),
                    net   : "\"unknown\"".to_string(),
                    path  : "\"unknown\"".to_string(),
                    port  : "\"unknown\"".to_string(),
                    ps    : "\"unknown\"".to_string(),
                    tls   : "\"unknown\"".to_string(),
                    typpe : "\"unknown\"".to_string()

                }
            },
            Tcp::V2 => {
                let newurl=&url[8..];
                let json = ascii_to_string(base64::decode(newurl.to_string().as_bytes()).unwrap());
                let v : Result<Value> = serde_json::from_str(json.as_str());
                match v {
                    Ok(input)=>{
                        return MyButton{
                            //company : input["add"].to_string(),
                            urls : url,
                            func : "\"v2\"".to_string(),
                            add : input["add"].to_string(),
                            aid : input["aid"].to_string(),
                            host : input["host"].to_string(),
                            id : input["id"].to_string(),
                            net : input["net"].to_string(),
                            path : input["path"].to_string(),
                            port : input["port"].to_string(),
                            ps : input["ps"].to_string(),
                            tls : input["tls"].to_string(),
                            typpe : input["type"].to_string()
                        }}
                    Err(_)=>{
                        return MyButton{
                            urls  : url,
                            func  : "\"v2\"".to_string(),
                            add   : "\"unknown\"".to_string(),
                            aid   : "\"unknown\"".to_string(),
                            host  : "\"unknown\"".to_string(),
                            id    : "\"unknown\"".to_string(),
                            net   : "\"unknown\"".to_string(),
                            path  : "\"unknown\"".to_string(),
                            port  : "\"unknown\"".to_string(),
                            ps    : "\"unknown\"".to_string(),
                            tls   : "\"unknown\"".to_string(),
                            typpe : "\"unknown\"".to_string()

                        }

                    }
                }

                
            }
        }
    }
} 


