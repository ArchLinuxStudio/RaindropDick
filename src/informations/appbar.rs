use crossterm:: event::{self, Event, KeyCode};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{BarChart, Block, Borders},
    Frame, Terminal,
};
use crate::app::App;
use crate::state::Page;
pub struct AppBar<'a> {
    data: Vec<(&'a str, u64)>,
    last_tick: Instant,
}

impl<'a> AppBar<'a> {
    pub fn new() -> AppBar<'a> {
        AppBar {
            data: vec![
                ("B1", 9),
                ("B2", 12),
                ("B3", 5),
                ("B4", 8),
                ("B5", 2),
                ("B6", 4),
                ("B7", 5),
                ("B8", 9),
                ("B9", 14),
                ("B10", 15),
                ("B11", 1),
                ("B12", 0),
                ("B13", 4),
                ("B14", 6),
                ("B15", 4),
                ("B16", 6),
                ("B17", 4),
                ("B18", 7),
                ("B19", 13),
                ("B20", 8),
                ("B21", 11),
                ("B22", 9),
                ("B23", 3),
                ("B24", 5),
            ],
            last_tick:Instant::now(),
        }
    }

    fn on_tick(&mut self) {
        let value = self.data.pop().unwrap();
        self.data.insert(0, value);
    }
}
impl<'a> App for AppBar<'a> {
    fn run_app_local<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<crate::state::IFEXIT> {
        terminal.draw(|f| ui(f, self))?; 
        let tick_rate = Duration::from_millis(250);
        //let mut last_tick = Instant::now();
        let timeout = tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(crate::state::IFEXIT::Exit),
                    KeyCode::Char('t') => return Ok(crate::state::IFEXIT::Change(Page::SubScribe)),
                    _ => {}
                }
            }
        }
        if self.last_tick.elapsed() >= tick_rate {
            self.on_tick();
            self.last_tick = Instant::now();
        }
        Ok(crate::state::IFEXIT::Next)

    }
}


fn ui<B: Backend>(f: &mut Frame<B>, app: &AppBar) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());
    let barchart = BarChart::default()
        .block(Block::default().title("Data1").borders(Borders::ALL))
        .data(&app.data)
        .bar_width(9)
        .bar_style(Style::default().fg(Color::Yellow))
        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));
    f.render_widget(barchart, chunks[0]);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let barchart = BarChart::default()
        .block(Block::default().title("Data2").borders(Borders::ALL))
        .data(&app.data)
        .bar_width(5)
        .bar_gap(3)
        .bar_style(Style::default().fg(Color::Green))
        .value_style(
            Style::default()
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(barchart, chunks[0]);

    let barchart = BarChart::default()
        .block(Block::default().title("Data3").borders(Borders::ALL))
        .data(&app.data)
        .bar_style(Style::default().fg(Color::Red))
        .bar_width(7)
        .bar_gap(0)
        .value_style(Style::default().bg(Color::Red))
        .label_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        );
    f.render_widget(barchart, chunks[1]);
}
