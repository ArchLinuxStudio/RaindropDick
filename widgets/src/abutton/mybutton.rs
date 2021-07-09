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
    pub urls : String,
    pub name : String,
    pub port :String,
    pub func :String,
    pub company:String,

}

impl MyButton{
    pub fn output(&self) -> Dialog {
        Dialog::text(format!("Name:{}\nUrl:{}\nport:{}\nfunction:{}\ncompany:{}", self.name,self.urls,self.port,self.func,self.company))
                .title(format!("{}", self.company))
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
                    urls : url,
                    name : "unknown".to_string(),
                    port : "unknown".to_string(),
                    func : "ss".to_string(),
                    company : "unknown".to_string(),
                }
            },
            Tcp::V2 => {
                let newurl=&url[8..];
                let json = ascii_to_string(base64::decode(newurl.to_string().as_bytes()).unwrap());
                let v : Result<Value> = serde_json::from_str(json.as_str());
                match v {
                    Ok(input)=>{
                    return MyButton{
                        name : input["ps"].to_string(),
                        urls: url,
                        port : input["port"].to_string(),
                        func : "v2".to_string(),
                        company : input["add"].to_string(),
                
                    }}
                    Err(_)=>{
                        return MyButton{
                            name : "ps".to_string(),
                            urls: url,
                            port : "port".to_string(),
                            func : "v2".to_string(),
                            company :"add".to_string(),
                        }
                    }
                }

                
            }
        }
    }
} 


