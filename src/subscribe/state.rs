use super::appsub::*;
use super::spider;
use super::utils;
use super::{Page, IFEXIT};
use crossterm::event::{self, Event, KeyCode};
use std::{env, io, process::Command};
pub(crate) fn subscribe_state(app: &mut AppSub) -> io::Result<IFEXIT> {
    if let Event::Key(key) = event::read()? {
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
                    let input = vec![app.input.clone()];
                    let get_list = spider::get_the_key(input);
                    if let Ok(list) = get_list {
                        let mut storge: String = String::new();
                        storge.push('[');
                        storge.push('\n');
                        if !list[0].is_empty() {
                            //app.subs = list[0].clone();
                            app.stateoflist = true;
                            app.state.select(Some(0));
                            for alist in &list[0] {
                                let information = spider::Information::new(alist.to_string());
                                app.informations.push(information.clone());
                                storge.push_str(information.get_the_json_node().as_str());
                            }
                            app.subs = app
                                .informations
                                .iter()
                                .map(|ainformation| {
                                    spider::remove_quotation(ainformation.ps.clone())
                                })
                                .collect();
                        }
                        storge.pop();
                        storge.pop();
                        storge.push('\n');
                        storge.push(']');
                        utils::create_json_file(utils::Save::Storage, storge)
                            .unwrap_or_else(|err| panic!("err {}", err));
                    }

                    //app.subs.push(app.input.drain(..).collect());
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
                        KeyCode::Esc => {
                            app.unselect();
                            app.input_mode = InputMode::Normal;
                        }
                        KeyCode::F(5) => {
                            if let Some(index) = app.state.selected() {
                                let home = env::var("HOME").unwrap();
                                utils::create_json_file(
                                    utils::Save::Running,
                                    app.informations[index].clone().running_json(),
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
                        format!(
                            "{{\n   \
                                    \"v2core\":\"{}\"\n\
                            }}",
                            app.settings_input[0]
                        ),
                    )
                    .unwrap_or_else(|err| panic!("{}", err));
                    let mut subscribe_json: String = "[\n\n".to_string();
                    for asub in &app.subscription {
                        subscribe_json.push_str(&format!(
                            "{{ \n\
                                \"url\": \"{}\"\n   \
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
                    let get_list = spider::get_the_key(app.subscription.clone());
                    if let Ok(list) = get_list {
                        let mut storge: String = String::new();
                        storge.push('[');
                        storge.push_str("\n\n");
                        if !list.is_empty() && !list[0].is_empty() {
                            //app.subs = list[0].clone();
                            app.stateoflist = true;
                            app.state.select(Some(0));
                            for alist in &list[0] {
                                let information = spider::Information::new(alist.to_string());
                                app.informations.push(information.clone());
                                storge.push_str(information.get_the_json_node().as_str());
                            }
                            app.subs = app
                                .informations
                                .iter()
                                .map(|ainformation| {
                                    spider::remove_quotation(ainformation.ps.clone())
                                })
                                .collect();
                        }
                        storge.pop();
                        storge.pop();
                        storge.push('\n');
                        storge.push(']');
                        utils::create_json_file(utils::Save::Storage, storge)
                            .unwrap_or_else(|err| panic!("err {}", err));
                    }
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
