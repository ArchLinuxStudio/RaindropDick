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
    match code {
        10 => '\n',
        13 => ' ',
        32 => ' ',
        34 => '\"',
        37 => '%',
        35 => '#',
        43 => '+',
        44 => ',',
        45 => '-',
        46 => '.',
        47 => '/',
        48 => '0',
        49 => '1',
        50 => '2',
        51 => '3',
        52 => '4',
        53 => '5',
        54 => '6',
        55 => '7',
        56 => '8',
        57 => '9',
        58 => ':',
        59 => ';',
        60 => '<',
        61 => '=',
        62 => '>',
        63 => '?',
        64 => '@',
        65 => 'A',
        66 => 'B',
        67 => 'C',
        68 => 'D',
        69 => 'E',
        70 => 'F',
        71 => 'G',
        72 => 'H',
        73 => 'I',
        74 => 'J',
        75 => 'K',
        76 => 'L',
        77 => 'M',
        78 => 'N',
        79 => 'O',
        80 => 'P',
        81 => 'Q',
        82 => 'R',
        83 => 'S',
        84 => 'T',
        85 => 'U',
        86 => 'V',
        87 => 'W',
        88 => 'X',
        89 => 'Y',
        90 => 'Z',
        92 => '\\',
        97 => 'a',
        98 => 'b',
        99 => 'c',
        100 => 'd',
        101 => 'e',
        102 => 'f',
        103 => 'g',
        104 => 'h',
        105 => 'i',
        106 => 'j',
        107 => 'k',
        108 => 'l',
        109 => 'm',
        110 => 'n',
        111 => 'o',
        112 => 'p',
        113 => 'q',
        114 => 'r',
        115 => 's',
        116 => 't',
        117 => 'u',
        118 => 'v',
        119 => 'w',
        120 => 'x',
        121 => 'y',
        122 => 'z',
        123 => '{',
        125 => '}',
        _ => '_',
    }
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
