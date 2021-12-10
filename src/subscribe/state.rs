use super::appsub::*;
use super::spider;
use super::utils;
use super::{Page, IFEXIT};
use crossterm::event::{self, Event, KeyCode};
use std::{env, io, process::Command};
use tui::widgets::ListState;
use serde_json::json;
pub(super) async fn subscribe_state(app: &mut AppSub) -> io::Result<IFEXIT> {
    if app.receiver.is_some() {
        if let Ok(get_list) = app.receiver.as_mut().unwrap().try_recv() {
            if let Ok(list) = get_list {
                let mut storge: String = "[\n\n".to_string();
                let mut subs: Vec<Vec<String>> = Vec::new();
                let mut information: Vec<Vec<spider::Information>> = Vec::new();
                let mut state: Vec<ListState> = Vec::new();
                for lista in list {
                    let mut ainformation: Vec<spider::Information> = Vec::new();
                    //let mut asub: Vec<String> = Vec::new();
                    storge.push_str("[\n\n");
                    if !lista.is_empty() {
                        for alist in lista {
                            let inform = spider::Information::new(alist.to_string());
                            ainformation.push(inform.clone());
                            storge.push_str(&inform.get_the_json_node());
                        }
                        storge.pop();
                        storge.pop();
                        storge.push_str("\n  ],");
                    }
                    state.push(ListState::default());
                    subs.push(
                        ainformation
                            .iter()
                            .map(|ainfor| ainfor.ps.clone())
                            .collect(),
                    );
                    information.push(ainformation);
                }
                app.state = state;
                app.subs = subs;
                app.informations = information;
                storge.pop();
                storge.push_str("\n]");
                utils::create_json_file(utils::Save::Storage, storge)
                    .unwrap_or_else(|err| panic!("err {}", err));
                app.subsindex = 0;
                app.stateoflist = true;
            }
            app.receiver = None;
            app.popinfomation = "Settings, e to edit, s to save".to_string();
            app.input_mode = InputMode::Popup;
        }
    } else if let Event::Key(key) = event::read()? {
        match app.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::Editing;
                }
                KeyCode::Char('s') => {
                    app.input_mode = InputMode::Select;
                }
                KeyCode::Char('q') => {
                    return Ok(IFEXIT::Exit);
                }
                KeyCode::Char('h') => {
                    //app.input = app.index.as_ref().unwrap().to_string();
                    app.show_popup = true;
                    app.input_mode = InputMode::Popup;
                }
                KeyCode::Char('t') | KeyCode::Char('2') => {
                    return Ok(IFEXIT::Change(Page::Information));
                }

                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    app.input = "This should be a search bar".to_string();
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Esc => {
                    app.input_mode = InputMode::Normal;
                }
                _ => {}
            },
            InputMode::Select => {
                if app.stateoflist {
                    match key.code {
                        //KeyCode::Left => app.unselect(),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Left => app.left(),
                        KeyCode::Right => app.right(),
                        KeyCode::Esc => {
                            app.unselect();
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::F(5) => {
                            if let Some(index) = app.state[app.subsindex].selected() {
                                let home = env::var("HOME").unwrap();
                                utils::create_json_file(
                                    utils::Save::Running,
                                    app.informations[app.subsindex][index]
                                        .clone()
                                        .running_json(),
                                )
                                .unwrap_or_else(|err| panic!("err {}", err));
                                Command::new("pkill")
                                    .arg("v2ray")
                                    .output()
                                    .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
                                Command::new("nohup")
                                    .arg(app.settings_input[0].clone())
                                    .arg("-config")
                                    .arg(home.clone() + "/.config/tv2ray/running.json")
                                    .arg(">")
                                    .arg(home + "/.config/tv2ray/test.log")
                                    .arg("2>&1")
                                    .arg("&")
                                    .spawn()
                                    .expect("failed");
                            }
                        }
                        _ => {}
                    }
                } else {
                    app.input_mode = InputMode::Normal;
                }
            }
            InputMode::Popup => match key.code {
                KeyCode::Char('q') => {
                    app.input_mode = InputMode::Normal;
                    app.show_popup = false;
                }
                KeyCode::Char('e') => {
                    app.input_mode = InputMode::PopupEdit;
                }

                KeyCode::Char('s') => {
                    utils::create_json_file(
                        utils::Save::V2ray,
                        json!({
                            "v2core" : app.settings_input[0]
                        }).to_string()
                    )
                    .unwrap_or_else(|err| panic!("{}", err));
                    let mut subscribe_json: String = "[\n\n".to_string();
                    for asub in &app.subscription {
                        subscribe_json.push_str(&format!(
                            "{{\n   \
                                \"url\": \"{}\"\n\
                            }},\n",
                            asub
                        ));
                    }
                    subscribe_json.pop();
                    subscribe_json.pop();
                    subscribe_json.push_str("\n]");
                    utils::create_json_file(utils::Save::Subscribes, subscribe_json)
                        .unwrap_or_else(|err| panic!("{}", err));
                    //    .collect();
                    let (sync_io_tx, sync_io_rx) =
                        tokio::sync::mpsc::channel::<reqwest::Result<Vec<Vec<String>>>>(100);
                    app.receiver = Some(sync_io_rx);
                    let input = app.subscription.clone();
                    app.popinfomation = "Waiting for a moment".to_string();
                    tokio::spawn(async move {
                        let get_list = spider::get_the_key(input).await;
                        sync_io_tx.send(get_list).await.unwrap();
                    });
                }
                _ => {}
            },
            InputMode::PopupEdit => {
                match key.code {
                    KeyCode::Esc => app.input_mode = InputMode::Popup,
                    // here todo
                    KeyCode::Char(c) => {
                        app.settings_input[app.index_settings].push(c);
                    }
                    KeyCode::Backspace => {
                        app.settings_input[app.index_settings].pop();
                    }
                    KeyCode::Down => {
                        if app.index_settings == 0 {
                            app.index_settings = 1;
                        } else if !app.subscription.is_empty() {
                            app.input_mode = InputMode::SubscriptView;
                            app.index_subscription.select(Some(0));
                        } else {
                            app.index_settings = 0;
                        }
                    }
                    KeyCode::Up => {
                        if app.index_settings == 1 {
                            app.index_settings = 0;
                        } else if !app.subscription.is_empty() {
                            app.input_mode = InputMode::SubscriptView;
                            app.index_subscription.select(Some(0));
                        } else {
                            app.index_settings = 1;
                        }
                    }
                    KeyCode::Enter => {
                        app.subscription.push(app.settings_input[1].clone());
                    }
                    _ => {}
                }
            }
            InputMode::SubscriptView => match key.code {
                KeyCode::Up => app.previous_sub(),
                KeyCode::Down => app.next_sub(),
                KeyCode::Char('d') => {
                    app.subscription
                        .remove(app.index_subscription.selected().unwrap());
                    if app.subscription.is_empty() {
                        app.unselect_sub();
                        app.input_mode = InputMode::Popup;
                    } else {
                        app.index_subscription.select(Some(0));
                    }
                    //app.settings_input[app.index_settings].push(c);
                }
                KeyCode::Esc => {
                    app.index_settings = 0;
                    app.unselect_sub();
                    app.input_mode = InputMode::PopupEdit;
                }
                _ => {}
            },
        }
    }
    Ok(IFEXIT::Next)
}
