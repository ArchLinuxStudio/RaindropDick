use crate::state::IFEXIT;
use std::io;
use tui::{backend::Backend, Terminal};

pub trait App {
    fn run_app_local<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<IFEXIT>;
}
