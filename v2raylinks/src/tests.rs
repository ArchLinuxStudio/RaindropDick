use super::*;
#[test]
fn tst_serde() {
    let a = Links::VMESS(Vmess {
        v: "2".to_string(),
        ps: "ss".to_string(),
        add: "".to_string(),
        port: "".to_string(),
        id: "".to_string(),
        aid: "".to_string(),
        net: "".to_string(),
        vmtype: "".to_string(),
        host: "".to_string(),
        path: "".to_string(),
        tls: "".to_string(),
        sni: None,
    });
    assert_eq!(
        r#"{"VMESS":{"v":"2","ps":"ss","add":"","port":"","id":"","aid":"","net":"","type":"","host":"","path":"","tls":"","sni":null}}"#,
        serde_json::to_string(&a).unwrap()
    );
}
#[test]
fn tst_enum() {
    let links = Links::new("ss://YWVzLTI1Ni1nY206ZDBkZWMwZWEtZjI4Zi00YzA1LTkwYTYtMTE0NTE0@127.0.0.1:8888#%E8%AB%8B%E6%AF%8F%E6%97%A5%E8%87%B3%E5%B0%91%E6%9B%B4%E6%96%B0%E4%B8%80%E6%AC%A1%E8%A8%82%E9%96%B2%20%E8%8B%A5%E6%9B%B4%E6%96%B0%E5%BE%8C%E5%A4%9A%E6%95%B8%E7%AF%80%E9%BB%9E%E4%BE%9D%E7%84%B6%E7%84%A1%E6%B3%95%E9%80%A3%E6%8E%A5%EF%BC%8C%E8%AB%8B%E7%99%BB%E5%85%A5%E5%AE%98%E7%B6%B2%E6%A3%80%E6%9F%A5%E5%A5%97%E9%A4%90%E7%8A%B6%E6%80%81");
    assert_eq!(
        links,
        Links::SS(SS {
            method: "aes-256-gcm".to_string(),
            passward: "d0dec0ea-f28f-4c05-90a6-114514".to_string(),
            ip: "127.0.0.1".to_string(),
            port: "8888".to_string(),
            linkname: "請每日至少更新一次訂閲 若更新後多數節點依然無法連接，請登入官網检查套餐状态"
                .to_string()
        })
    )
}
#[test]
fn tst_ssname() {
    assert!(SSTIP.is_match("%22%22"));
    assert!(SSTIP.is_match("%2A%22"));
    assert!(SSTIP.is_match("%2A2%22"));
    assert!(SSTIP.is_match("%2A2%22Abcd"));
    let origin = "%E8%AB%8B%E6%AF%8F%E6%97%A5%E8%87%B3%E5%B0%91%E6%9B%B4%E6%96%B0%E4%B8%80%E6%AC%A1%E8%A8%82%E9%96%B2%20%E8%8B%A5%E6%9B%B4%E6%96%B0%E5%BE%8C%E5%A4%9A%E6%95%B8%E7%AF%80%E9%BB%9E%E4%BE%9D%E7%84%B6%E7%84%A1%E6%B3%95%E9%80%A3%E6%8E%A5%EF%BC%8C%E8%AB%8B%E7%99%BB%E5%85%A5%E5%AE%98%E7%B6%B2%E6%A3%80%E6%9F%A5%E5%A5%97%E9%A4%90%E7%8A%B6%E6%80%81ababc";
    //let origin = "%E6";
    let mut toinput: Vec<u8> = vec![];

    for caps in UTF8UNIT.captures_iter(origin) {
        toinput.push(u8::from_str_radix(caps.get(1).unwrap().as_str(), 16).unwrap());
    }
    let length = 3 * toinput.len();
    assert_eq!(
        String::from_utf8(toinput).unwrap(),
        "請每日至少更新一次訂閲 若更新後多數節點依然無法連接，請登入官網检查套餐状态"
    );
    let tips = &origin[length..];
    assert_eq!("ababc", tips);
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
        "127.0.0.1"
    );
    assert_eq!(
        &SSLINK_WITHINFO
            .captures("ss://abc@127.0.0.1:114514#steingate")
            .unwrap()["Ip"],
        "127.0.0.1"
    );
    assert_eq!(
        &SSLINK_WITHINFO
            .captures("ss://abc@127.0.0.1:114514#steingate")
            .unwrap()["Port"],
        "114514"
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
#[test]
fn tst_baselink() {
    assert!(BASELINK.is_match("ss://1234"));
    let links = include_str!("../assert/decodekeys.txt");
    let _links = BASELINK
        .captures_iter(links)
        .map(|unit| Links::new(unit.get(1).unwrap().as_str()))
        .collect::<Vec<Links>>();
}
