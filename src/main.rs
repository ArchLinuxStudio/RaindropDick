/// A simple example demonstrating how to handle user input. This is
/// a bit out of the scope of the library as it does not provide any
/// input handling out of the box. However, it may helps some to get
/// started.
///
/// This is a very simple example:
///   * A input box always focused. Every character you type is registered
///   here
///   * Pressing Backspace erases a character
///   * Pressing Enter pushes the current input in the history of previous
///   messages
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
mod spider;
mod utils;
enum InputMode {
    Normal,
    Editing,
    Select,
    Popup,
    PopupEdit,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    v2ray_input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
    state: ListState,
    index: Option<usize>,
    stateoflist: bool,
    show_popup: bool,
    informations: Vec<spider::Information>,
}
impl App {
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.messages.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.index = Some(i);
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.messages.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.index = Some(i);
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}
impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            v2ray_input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            state: ListState::default(),
            index: None,
            stateoflist: false,
            show_popup: false,
            informations: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::default();
    let informations = utils::start();
    if !informations.is_empty() {
        app.messages = informations
            .iter()
            .map(|amessage| amessage.urls.clone())
            .collect();
        app.stateoflist = true;
        app.state.select(Some(0));
        app.index = Some(0);
        app.informations = informations;
    }
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
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
                                app.messages = list[0].clone();
                                app.stateoflist = true;
                                app.state.select(Some(0));
                                app.index = Some(0);
                                for alist in &list[0] {
                                    let information = spider::Information::new(alist.to_string());
                                    app.informations.push(information.clone());
                                    storge.push_str(information.get_the_json_node().as_str());
                                }
                            }
                            storge.pop();
                            storge.pop();
                            storge.push('\n');
                            storge.push(']');
                            if let Err(err) = utils::create_json_file(utils::Save::Storage, storge)
                            {
                                panic!("err {}", err);
                            };
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
                    _ => {}
                },
                InputMode::PopupEdit => {
                    match key.code {
                        KeyCode::Esc => app.input_mode = InputMode::Popup,
                        // here todo
                        KeyCode::Char(c) => {
                            app.v2ray_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app.v2ray_input.pop();
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
        InputMode::Normal | InputMode::Popup | InputMode::PopupEdit => (
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
    if let Some(a) = app.index {
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
        let inputpop = Paragraph::new(app.v2ray_input.as_ref())
            .style(match app.input_mode {
                InputMode::PopupEdit => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .block(Block::default().borders(Borders::ALL).title("Settings"));
        //f.render_widget(input, chunks[1]);
        let area = centered_rect(60, 20, f.size());
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(inputpop, area);
        if let InputMode::PopupEdit = app.input_mode {
            // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                area.x + app.v2ray_input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                area.y + 1,
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
