extern crate base64;
use reqwest::Result;
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

