use crate::state::{MyBackend, IFEXIT};
use std::io;
use tui::Terminal;
use async_trait::async_trait;
#[async_trait]
pub trait App {
    async fn run_app_local(&mut self, terminal: &mut Terminal<MyBackend>) -> io::Result<IFEXIT>;
}
