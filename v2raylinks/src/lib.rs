use once_cell::sync::Lazy;
use serde::Deserialize;
pub static SSLINK_WITHINFO: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"^ss://(?P<MethodAndPassward>[^@]+)@(?P<Ip>[^#]+)#(?P<Tips>.+)$").unwrap()
});
pub static SSLINK: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^ss://(?P<MethodAndPassWard>[^@]+)@(?P<Ip>[^#]+)$").unwrap());
pub static SSMETHODANDPASSWARD: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^(?P<Method>[^:]+):(?P<Passward>.+)$").unwrap());
pub static VMESSLINK: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^vmess://(.+)$").unwrap());
// FIXME
pub static SSTIP: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"^[%\d{2}]+$").unwrap());
pub static UTF8UNIT: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"%([\da-zA-Z]{2})").unwrap());
pub static SSTIPWITHINFO: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^[%\d{2}]+(.+)$").unwrap());
#[derive(Clone, Deserialize)]
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
#[test]
fn tst_ssname() {
    assert!(SSTIP.is_match("%22%22"));
    assert!(!SSTIP.is_match("%2s2%22"));
    assert!(SSTIPWITHINFO.is_match("%22%22abc"));
    let origin = "%E8%AB%8B%E6%AF%8F%E6%97%A5%E8%87%B3%E5%B0%91%E6%9B%B4%E6%96%B0%E4%B8%80%E6%AC%A1%E8%A8%82%E9%96%B2%20%E8%8B%A5%E6%9B%B4%E6%96%B0%E5%BE%8C%E5%A4%9A%E6%95%B8%E7%AF%80%E9%BB%9E%E4%BE%9D%E7%84%B6%E7%84%A1%E6%B3%95%E9%80%A3%E6%8E%A5%EF%BC%8C%E8%AB%8B%E7%99%BB%E5%85%A5%E5%AE%98%E7%B6%B2%E6%A3%80%E6%9F%A5%E5%A5%97%E9%A4%90%E7%8A%B6%E6%80%81";
    //let origin = "%E6";
    let mut toinput : Vec<u8> = vec![];
    for caps in UTF8UNIT.captures_iter(origin) {
        toinput.push(u8::from_str_radix(caps.get(1).unwrap().as_str(), 16).unwrap());
    }
    assert_eq!(String::from_utf8(toinput).unwrap(),"請每日至少更新一次訂閲 若更新後多數節點依然無法連接，請登入官網检查套餐状态");

}
#[test]
fn tst_link() {
    assert!(SSLINK_WITHINFO.is_match("ss://abcd@127.0.0.1:114514#steingate"));
    assert!(!SSLINK_WITHINFO.is_match("ssr://abcd"));
    assert!(VMESSLINK.is_match("vmess://ssss"));
    assert!(!VMESSLINK.is_match("vmes://abcd"));
    assert_eq!(
        VMESSLINK
            .captures("vmess://abcd")
            .unwrap()
            .get(1)
            .unwrap()
            .as_str(),
        "abcd"
    );
    assert_eq!(
        SSLINK_WITHINFO
            .captures("ss://abc@127.0.0.1:114514#steingate")
            .unwrap()
            .get(1)
            .unwrap()
            .as_str(),
        "abc"
    );
    assert_eq!(
        SSLINK_WITHINFO
            .captures("ss://abc@127.0.0.1:114514#steingate")
            .unwrap()
            .get(2)
            .unwrap()
            .as_str(),
        "127.0.0.1:114514"
    );
    assert_eq!(
        &SSLINK_WITHINFO
            .captures("ss://abc@127.0.0.1:114514#steingate")
            .unwrap()["Ip"],
        "127.0.0.1:114514"
    );
    assert!(SSMETHODANDPASSWARD.is_match("aes-256-gcm:d0dec0ea-f28f-4c05-90a6-8e1449c71aa1"));
    assert_eq!(
        &SSMETHODANDPASSWARD
            .captures("aes-256-gcm:d0dec0ea-f28f-4c05-90a6-8e1449c71aa1")
            .unwrap()["Method"],
        "aes-256-gcm"
    );
    assert_eq!(
        &SSMETHODANDPASSWARD
            .captures("aes-256-gcm:d0dec0ea-f28f-4c05-90a6-8e1449c71aa1")
            .unwrap()["Passward"],
        "d0dec0ea-f28f-4c05-90a6-8e1449c71aa1"
    );
}
#[test]
fn tst_decodevmess() {
    let link = r#"
    {
        "v":"2",
        "ps":"Steingate",
        "add":"11.45.11.14",
        "port":"13915",
        "id":"guess-it",
        "aid":"0",
        "net":"tcp",
        "type":"none",
        "host":"",
        "path":"",
        "tls":"tls",
        "sni":"Steingate.com"
    }
    "#;
    let temp: Vmess = serde_json::from_str(link).unwrap();
    assert_eq!("2", temp.v);
    assert_eq!("Steingate", temp.ps);
    assert_eq!("13915", temp.port);
}
