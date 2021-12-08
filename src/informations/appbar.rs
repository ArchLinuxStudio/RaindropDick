use crate::app::App;
use crate::state::MyBackend;
use std::{
    io,
    time::{Duration, Instant},
};
use tui::Terminal;
pub struct AppBar<'a> {
    pub(crate) data: Vec<(&'a str, u64)>,
    last_tick: Instant,
}
use super::render::ui;
use super::state::information_state;
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
            last_tick: Instant::now(),
        }
    }

    fn on_tick(&mut self) {
        let value = self.data.pop().unwrap();
        self.data.insert(0, value);
    }
}
impl<'a> App for AppBar<'a> {
    fn run_app_local(
        &mut self,
        terminal: &mut Terminal<MyBackend>,
    ) -> io::Result<crate::state::IFEXIT> {
        terminal.draw(|f| ui(f, self))?;
        let tick_rate = Duration::from_millis(250);
        //let mut last_tick = Instant::now();
        let timeout = tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if self.last_tick.elapsed() >= tick_rate {
            self.on_tick();
            self.last_tick = Instant::now();
        }
        information_state(timeout)
    }
}
