use futures::future::join_all;
extern crate base64;

async fn fetch_path(path:String) -> surf::Result<String>{
    let mut back_string = String::new();
    let url = surf::http::Url::parse(&path);
    match url{
        Ok(_)=>
        { match surf::get(&path).await {
                Ok(mut response) => {
                    match response.body_string().await{
                        Ok(text) =>{
                            back_string = text
                        }
                        Err(_) => {
                            println!("Read response text Error!")
                        }
                    };
                }
                Err(_) => {
                    println!("reqwest get Error!")
                }
            }
            return Ok(back_string);
        }
        Err(_)=>{
            return Ok(String::new());
        }
    }
}
pub fn ascii_to_char(code:u8) -> char {
    match code {
        10 => return '\n',
        13 => return ' ',
        34 => return '\"',
        37 => return '%',
        35 => return '#',
        43 => return '+',
        44 => return ',',
        45 => return '-',
        46 => return '.',
        47 => return '/',
        48 => return '0',
        49 => return '1',
        50 => return '2',
        51 => return '3',
        52 => return '4',
        53 => return '5',
        54 => return '6',
        55 => return '7',
        56 => return '8',
        57 => return '9',
        58 => return ':',
        59 => return ';',
        60 => return '<',
        61 => return '=',
        62 => return '>',
        63 => return '?',
        64 => return '@',
        65 => return 'A',
        66 => return 'B',
        67 => return 'C',
        68 => return 'D',
        69 => return 'E',
        70 => return 'F',
        71 => return 'G',
        72 => return 'H',
        73 => return 'I',
        74 => return 'J',
        75 => return 'K',
        76 => return 'L',
        77 => return 'M',
        78 => return 'N',
        79 => return 'O',
        80 => return 'P',
        81 => return 'Q',
        82 => return 'R',
        83 => return 'S',
        84 => return 'T',
        85 => return 'U',
        86 => return 'V',
        87 => return 'W',
        88 => return 'X',
        89 => return 'Y',
        90 => return 'Z',
        92 => return '\\',
        97 => return 'a',
        98 => return 'b',
        99 => return 'c',
        100 => return 'd',
        101 => return 'e',
        102 => return 'f',
        103 => return 'g',
        104 => return 'h',
        105 => return 'i',
        106 => return 'j',
        107 => return 'k',
        108 => return 'l',
        109 => return 'm',
        110 => return 'n',
        111 => return 'o',
        112 => return 'p',
        113 => return 'q',
        114 => return 'r',
        115 => return 's',
        116 => return 't',
        117 => return 'u',
        118 => return 'v',
        119 => return 'w',
        120 => return 'x',
        121 => return 'y',
        122 => return 'z',
        123 => return '{',
        125 => return '}',
        _ => {
            println!("{}",code);
            return '\n'
        },
    }
}
fn ascii_to_string(code:Vec<u8>) -> Vec<String>{
    let mut test:Vec<String> = vec![];
    let num = code.len();
    let mut count = 0;
    while count<num {
        let mut url = String::new();
        while code[count] !=13 && code[count] !=10 {
            url.push(ascii_to_char(code[count]));
            count=count+1;
        }
        test.push(url);
        if code[count] == 10 {
            count=count+1;
        } else {
            count = count+2;
        }
    }
    test
}

pub async fn get_the_key(paths:Vec<String>) -> surf::Result<Vec<Vec<String>>>{
    let result_list = join_all(paths.into_iter().map(|path|{
        fetch_path(path)
    })).await;

    let mut list_string:Vec<String> = vec![];
    for ele in result_list.into_iter(){
        if ele.is_ok(){
            list_string.push(ele.unwrap())
        }else {
            return Err(ele.unwrap_err())
        }
    }
    let url_string_none:Vec<Vec<String>> = vec![];
    let mut url_string:Vec<Vec<String>> = vec![];
    for pair in list_string.into_iter(){
        let code = base64::decode(pair.as_bytes());
        match code {
            Ok(input)=>{
                url_string.push(ascii_to_string(input));
            }
            Err(_)=>{
                return Ok(url_string_none);
            }
        }
        //}
    }
    Ok(url_string)
}

