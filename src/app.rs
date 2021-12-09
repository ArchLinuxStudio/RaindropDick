use crate::state::{MyBackend, IFEXIT};
use async_trait::async_trait;
use std::io;
use tui::Terminal;
#[async_trait]
pub trait App {
    async fn run_app_local(&mut self, terminal: &mut Terminal<MyBackend>) -> io::Result<IFEXIT>;
}
