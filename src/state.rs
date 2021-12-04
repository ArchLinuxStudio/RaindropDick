use crate::app::*;
use crate::subscribe::render;
use std::io;
use tui::{backend::Backend, Terminal};
pub enum IFEXIT {
    Next,
    Exit,
}
//计划将它设置成一个入口
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        // here ,need with different tab ,use different draw funcitons
        //terminal.draw(|f| render::ui(f, &mut app))?;
        if let IFEXIT::Exit = render::run_app_subscribe(terminal,&mut app)? {
            return Ok(());
        }
    }
}
