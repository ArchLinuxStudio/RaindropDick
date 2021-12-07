use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};

use crate::state::{Page, IFEXIT};
pub fn information_state(timeout: Duration) -> io::Result<IFEXIT> {
    if crossterm::event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(crate::state::IFEXIT::Exit),
                KeyCode::Char('t') => return Ok(crate::state::IFEXIT::Change(Page::SubScribe)),
                _ => return Ok(IFEXIT::Next),
            }
        }
    }
    Ok(IFEXIT::Next)
}
