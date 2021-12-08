use crate::state::{MyBackend, IFEXIT};
use std::io;
use tui::Terminal;

pub trait App {
    fn run_app_local(&mut self, terminal: &mut Terminal<MyBackend>) -> io::Result<IFEXIT>;
}
