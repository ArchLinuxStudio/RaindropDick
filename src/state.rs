use crate::app::*;
use crate::spider;
use crate::utils;
use crossterm::event::{self,KeyCode,Event};
use std::{env,  io, process::Command};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

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
                        return Ok(());
                    }
                    KeyCode::Char('h') => {
                        //app.input = app.index.as_ref().unwrap().to_string();
                        app.show_popup = true;
                        app.input_mode = InputMode::Popup;
                    }

                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let input = vec![app.input.clone()];
                        let get_list = spider::get_the_key(input.clone());
                        if let Ok(list) = get_list {
                            let mut storge: String = String::new();
                            storge.push('[');
                            storge.push('\n');
                            if !list[0].is_empty() {
                                //app.messages = list[0].clone();
                                app.stateoflist = true;
                                app.state.select(Some(0));
                                for alist in &list[0] {
                                    let information = spider::Information::new(alist.to_string());
                                    app.informations.push(information.clone());
                                    storge.push_str(information.get_the_json_node().as_str());
                                }
                                app.messages = app
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

                        //app.messages.push(app.input.drain(..).collect());
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
                            KeyCode::Left => app.unselect(),
                            KeyCode::Down => app.next(),
                            KeyCode::Up => app.previous(),
                            KeyCode::Esc => {
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
                                    Command::new("pkill").arg("v2ray").output().unwrap_or_else(
                                        |e| panic!("failed to execute process: {}", e),
                                    );
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
                        if let Err(err) = utils::create_json_file(
                            utils::Save::V2ray,
                            format!(
                                "{{
    \"v2core\":\"{}\"
}}",
                                app.settings_input[0]
                            ),
                        ) {
                            panic!("{}", err);
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
                            } else if !app.subscription.is_empty(){
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
                InputMode::SubscriptView => {
                    match key.code {
                        KeyCode::Up => app.previous_sub(),
                        KeyCode::Down => app.next_sub(),
                        KeyCode::Esc => {
                            app.index_settings =0;
                            app.unselect_sub();
                            app.input_mode = InputMode::PopupEdit;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let (msg, style) = match app.input_mode {
        InputMode::Normal | InputMode::Popup | InputMode::PopupEdit | InputMode::SubscriptView=> (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
                Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to select trees"),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Select => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Editing => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[1]);
    if let InputMode::Editing = app.input_mode {
        // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
        f.set_cursor(
            // Put cursor past the end of the input text
            chunks[1].x + app.input.width() as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[1].y + 1,
        )
        //InputMode::Normal | InputMode::Select | InputMode::Popup =>
        // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
    }

    // Bottom two inner blocks
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[2]);

    let messages: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();
    let messages = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    // popup wiget
    f.render_stateful_widget(messages, bottom_chunks[0], &mut app.state);
    //let block : Box<dyn Widget> = {
    if let Some(a) = app.state.selected() {
        let list = app.informations[a].information_to_list();
        let messages: Vec<ListItem> = list
            .iter()
            .map(|infom| {
                let content = vec![Spans::from(Span::raw(infom))];
                ListItem::new(content)
            })
            .collect();
        let block =
            List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
        f.render_widget(block, bottom_chunks[1]);
    } else {
        let block = Block::default().title("With borders").borders(Borders::ALL);
        f.render_widget(block, bottom_chunks[1]);
    }
    //};
    //f.render_widget(*block, bottom_chunks[0]);

    if app.show_popup {
        //let block = Block::default().title("About port").borders(Borders::ALL);

        //f.render_widget(input, chunks[1]);
        let area = centered_rect(80, 50, f.size());

        let chunk = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(area);

        f.render_widget(Clear, area); //this clears out the background
        let (msg, style) = (
            vec![Span::raw("Settings, e to edit, s to save")],
            Style::default(),
        );
        let mut text = Text::from(Spans::from(msg));
        text.patch_style(style);
        let title = Paragraph::new(text);
        f.render_widget(title, chunk[0]);
        let inputpop = Paragraph::new(app.settings_input[0].as_ref())
            .style(match (&app.input_mode, app.index_settings) {
                (InputMode::PopupEdit, 0) => Style::default().fg(Color::Yellow),
                (_, _) => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("v2ray-core"));
        f.render_widget(inputpop, chunk[1]);
        let inputpop2 = Paragraph::new(app.settings_input[1].as_ref())
            .style(match (&app.input_mode, app.index_settings) {
                (InputMode::PopupEdit, 1) => Style::default().fg(Color::Yellow),
                (_, _) => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("add domins"));
        f.render_widget(inputpop2, chunk[2]);
        let subscription : Vec<ListItem> = app
            .subscription
            .iter()
            .enumerate()
            .map(|(i,m)|{
                let content = vec![Spans::from(Span::raw(format!("{}:{}",i,m)))];
                ListItem::new(content)
            })
        .collect();
        let subscription = List::new(subscription)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .highlight_style(
            Style::default()
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
        f.render_stateful_widget(subscription, chunk[3], &mut app.index_subscription);
        if let InputMode::PopupEdit = app.input_mode {
            let index = app.index_settings;
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunk[index + 2].x + app.settings_input[index].width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunk[index + 1].y + 1,
            )
            //InputMode::Normal | InputMode::Select | InputMode::Popup =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        }
    }
}
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
