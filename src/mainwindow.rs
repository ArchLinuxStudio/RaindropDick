use crate::abutton::mybutton::MyButton;
use crate::spider::get_the_key;
use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{
    Button, Dialog, DummyView, EditView, LinearLayout, NamedView, ResizedView, ScrollView,
    SelectView,
};
use cursive::Cursive;
use futures::executor::block_on;
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use std::{env, fs};
fn create_storage_before() {
    let home = env::var("HOME").unwrap();
    fs::create_dir_all(home + "/.config/tv2ray").unwrap();
}
fn url_select() -> ResizedView<ScrollView<NamedView<SelectView<MyButton>>>> {
    let mut start = SelectView::<MyButton>::new().on_submit(on_submit);
    create_storage_before();
    let home = env::var("HOME").unwrap();
    let location = home + "/.config/tv2ray/storage.json";
    let path = Path::new(location.as_str());
    //let display = path.display();
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(_) => {
            let path2 = Path::new(location.as_str());
            let display2 = path2.display();
            let mut file2 = match File::create(&path2) {
                Err(why) => panic!("couldn't create {}: {}", display2, why.to_string()),
                Ok(file2) => file2,
            };
            let mut storge2: String = String::new();
            storge2.push_str("[]");
            // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
            if let Err(why) = file2.write_all(storge2.as_bytes()) {
                panic!("couldn't write to {}: {}", display2, why.to_string())
            }
            let path3 = Path::new(location.as_str());
            File::open(&path3).unwrap()
        }
        Ok(file) => file,
    };
    let mut ss = String::new();
    match file.read_to_string(&mut ss) {
        Err(_) => {}
        Ok(_) => {
            let v: Value = serde_json::from_str(ss.as_str()).unwrap();
            let mut index = 0;
            while v[index] != Value::Null {
                let the_url = v[index]["url"].to_string();
                let lenghth = the_url.len();
                let instore = &the_url[1..lenghth - 1];
                let url = MyButton {
                    urls: instore.to_string(),
                    func: v[index]["func"].to_string(),
                    add: v[index]["add"].to_string(),
                    aid: v[index]["aid"].to_string(),
                    host: v[index]["host"].to_string(),
                    id: v[index]["id"].to_string(),
                    net: v[index]["net"].to_string(),
                    path: v[index]["path"].to_string(),
                    port: v[index]["port"].to_string(),
                    ps: v[index]["ps"].to_string(),
                    tls: v[index]["tls"].to_string(),
                    typpe: v[index]["type"].to_string(),
                };
                let names = v[index]["ps"].to_string();
                start.add_item(names, url);
                index += 1;
            }
        }
    }
    start
        .with_name("select")
        .scrollable()
        .scroll_y(true)
        .fixed_size((60, 25))
}
pub fn url_buttons() -> Dialog {
    let select = url_select();
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(Button::new("Load", onload))
        .child(Button::new("Stop", |_| {
            Command::new("pkill")
                .arg("v2ray")
                .output()
                .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
        }))
        .child(DummyView)
        .child(Button::new("Quit", quit));
    Dialog::around(
        LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons),
    )
    .title("Fuck you GFW")
}
fn delete_name(s: &mut Cursive) {
    let select = s.find_name::<SelectView<MyButton>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(_) => {
            s.add_layer(
                Dialog::around(
                    LinearLayout::horizontal()
                        .child(Button::new("Sure", |s| {
                            let mut select = s.find_name::<SelectView<MyButton>>("select").unwrap();
                            match select.selected_id() {
                                None => {
                                    s.add_layer(Dialog::info("No name to remove"));
                                    s.pop_layer();
                                    s.pop_layer();
                                }
                                Some(focus) => {
                                    select.remove_item(focus);
                                    s.pop_layer();
                                }
                            }
                        }))
                        .child(Button::new("cancle", |s| {
                            s.pop_layer();
                        })),
                )
                .title("Sure?"),
            );
        }
    }
}

fn on_submit(s: &mut Cursive, name: &MyButton) {
    s.add_layer(name.output());
}
fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<MyButton>| {
            view.clear();
            let temp: Vec<String> = vec![name.to_string()];
            //temp.push(name.to_string());
            let future = get_the_key(temp);
            let output: Vec<Vec<String>> = block_on(future).unwrap();
            let mut storge: String = String::new();
            storge.push('[');
            storge.push('\n');
            for urls in output.into_iter() {
                for url in urls.into_iter() {
                    let temp2 = MyButton::new(url);
                    storge.push_str(
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
                            temp2.clone().func,
                            temp2.clone().urls,
                            temp2.clone().add,
                            temp2.clone().aid,
                            temp2.clone().host,
                            temp2.clone().id,
                            temp2.clone().net,
                            temp2.clone().path,
                            temp2.clone().port,
                            temp2.clone().ps,
                            temp2.clone().tls,
                            temp2.clone().typpe
                        )
                        .as_str(),
                    );
                    let names = temp2.clone().ps;
                    view.add_item(names, temp2);
                }
            }
            storge.pop();
            storge.pop();
            storge.push('\n');
            storge.push(']');

            //Get the Home
            let home = env::var("HOME").unwrap();
            let location = home + "/.config/tv2ray/storage.json";
            let path2 = Path::new(location.as_str());
            //let display = path.display();
            //let path2 = Path::new("storage.json");
            let display2 = path2.display();
            let mut file2 = match File::create(&path2) {
                Err(why) => panic!("couldn't create {}: {}", display2, why.to_string()),
                Ok(file2) => file2,
            };

            // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
            if let Err(why) = file2.write_all(storge.as_bytes()) {
                panic!("couldn't write to {}: {}", display2, why.to_string())
            }
        });
        s.pop_layer();
    }

    s.add_layer(
        Dialog::around(
            EditView::new()
                .on_submit(ok)
                .with_name("input the urls")
                .fixed_width(70)
                .scrollable(),
        )
        .title("Enter a new name")
        .button("Ok", |s| {
            let name = s
                .call_on_name("input the urls", |view: &mut EditView| view.get_content())
                .unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }),
    );
}
fn quit(s: &mut Cursive) {
    Cursive::quit(s);
}
pub fn v2core() -> Dialog {
    fn ok(_s: &mut Cursive, name: &str) {
        let output: String = format!(
            "{{
    \"v2core\":\"{}\"
}}",
            name
        );
        let home = env::var("HOME").unwrap();
        let location = home + "/.config/tv2ray/v2core.json";
        let path2 = Path::new(location.as_str());
        //let display = path.display();
        //let path2 = Path::new("storage.json");
        let display2 = path2.display();
        let mut file2 = match File::create(&path2) {
            Err(why) => panic!("couldn't create {}: {}", display2, why.to_string()),
            Ok(file2) => file2,
        };

        // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
        if let Err(why) = file2.write_all(output.as_bytes()) {
            panic!("couldn't write to {}: {}", display2, why.to_string())
        }
    }
    create_storage_before();
    let home = env::var("HOME").unwrap();
    let location = home + "/.config/tv2ray/v2core.json";
    let path = Path::new(location.as_str());
    //let display = path.display();
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(_) => {
            let path2 = Path::new(location.as_str());
            let display2 = path2.display();
            let mut file2 = match File::create(&path2) {
                Err(why) => panic!("couldn't create {}: {}", display2, why.to_string()),
                Ok(file2) => file2,
            };
            let mut storge2: String = String::new();
            storge2.push_str("{\n\"v2core\":\"/usr/bin/v2ray\"\n}");
            // 将 `LOREM_IPSUM` 字符串写进 `file`，返回 `io::Result<()>`
            if let Err(why) = file2.write_all(storge2.as_bytes()) {
                panic!("couldn't write to {}: {}", display2, why.to_string())
            }
            let path3 = Path::new(location.as_str());
            File::open(&path3).unwrap()
        }
        Ok(file) => file,
    };
    let mut ss = String::new();
    let mut content: String = String::new();
    match file.read_to_string(&mut ss) {
        Err(_) => {}
        Ok(_) => {
            let v: Value = serde_json::from_str(ss.as_str()).unwrap();
            let temp = v["v2core"].to_string();
            let length = temp.len();
            content = (&temp[1..length - 1]).to_string();
        }
    }

    Dialog::around(
        EditView::new()
            .content(content)
            .on_submit(ok)
            .with_name("v2core")
            .fixed_width(30),
    )
    .title("Enter a new name")
    .button("Ok", |s| {
        let name = s
            .call_on_name("v2core", |view: &mut EditView| view.get_content())
            .unwrap();
        ok(s, &name);
        s.pop_layer();
    })
    .button("Cancel", |s| {
        s.pop_layer();
    })
}
fn onload(s: &mut Cursive) {
    let home = env::var("HOME").unwrap();
    let location = home + "/.config/tv2ray/storage.json";
    let path = Path::new(location.as_str());
    let display = path.display();
    let mut file = match File::open(&path) {
        // `io::Error` 的 `description` 方法返回一个描述错误的字符串。
        Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };
    let mut ss = String::new();
    match file.read_to_string(&mut ss) {
        Err(why) => {
            s.add_layer(Dialog::info(why.to_string()));
        }
        Ok(_) => {
            s.call_on_name("select", |view: &mut SelectView<MyButton>| {
                view.clear();
                let v: Value = serde_json::from_str(ss.as_str()).unwrap();
                let mut index = 0;
                while v[index] != Value::Null {
                    let url = MyButton {
                        urls: v[index]["url"].to_string(),
                        func: v[index]["func"].to_string(),
                        add: v[index]["add"].to_string(),
                        aid: v[index]["aid"].to_string(),
                        host: v[index]["host"].to_string(),
                        id: v[index]["id"].to_string(),
                        net: v[index]["net"].to_string(),
                        path: v[index]["path"].to_string(),
                        port: v[index]["path"].to_string(),
                        ps: v[index]["ps"].to_string(),
                        tls: v[index]["tls"].to_string(),
                        typpe: v[index]["type"].to_string(),
                    };
                    let names = v[index]["ps"].to_string();
                    view.add_item(names, url);
                    index += 1;
                }
            });
        }
    }
}
