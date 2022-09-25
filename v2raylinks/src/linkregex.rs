use once_cell::sync::Lazy;
pub static SSLINK_WITHINFO: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(
        r"^ss://(?P<MethodAndPassward>[^@]+)@(?P<Ip>[^#:]+):(?P<Port>[^#:]+)#(?P<Tips>.+)$",
    )
    .unwrap()
});
#[allow(dead_code)]
pub static SSLINK: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^ss://(?P<MethodAndPassward>[^@]+)@(?P<Ip>[^#]+)$").unwrap());
pub static SSMETHODANDPASSWARD: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^(?P<Method>[^:]+):(?P<Passward>.+)$").unwrap());
pub static VMESSLINK: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"^vmess://(.+)$").unwrap());
pub static UTF8UNIT: Lazy<regex::Regex> =
    Lazy::new(|| regex::Regex::new(r"%([\da-fA-F]{2})").unwrap());
#[allow(dead_code)]
pub static SSTIP: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"%[\da-zA-Z]{2}+").unwrap());
