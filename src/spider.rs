use futures::future::join_all;
extern crate base64;

async fn fetch_path(path: String) -> surf::Result<String> {
    let mut back_string = String::new();
    let url = surf::http::Url::parse(&path);
    match url {
        Ok(_) => {
            match surf::get(&path).await {
                Ok(mut response) => {
                    match response.body_string().await {
                        Ok(text) => back_string = text,
                        Err(_) => {
                            println!("Read response text Error!")
                        }
                    };
                }
                Err(_) => {
                    println!("reqwest get Error!")
                }
            }
            Ok(back_string)
        }
        Err(_) => Ok(String::new()),
    }
}
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

pub async fn get_the_key(paths: Vec<String>) -> surf::Result<Vec<Vec<String>>> {
    let result_list = join_all(paths.into_iter().map(fetch_path)).await;

    let mut list_string: Vec<String> = vec![];
    for ele in result_list.into_iter() {
        if ele.is_ok() {
            list_string.push(ele.unwrap())
        } else {
            return Err(ele.unwrap_err());
        }
    }
    let url_string_none: Vec<Vec<String>> = vec![];
    let mut url_string: Vec<Vec<String>> = vec![];
    for pair in list_string.into_iter() {
        let code = base64::decode(pair.as_bytes());
        match code {
            Ok(input) => {
                url_string.push(ascii_to_string(input));
            }
            Err(_) => {
                return Ok(url_string_none);
            }
        }
        //}
    }
    Ok(url_string)
}
