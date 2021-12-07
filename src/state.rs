use crate::app::App;
use crate::spider;
use crate::subscribe::appsub::AppSub;
use crate::utils;
use std::io;
use tui::{backend::Backend, Terminal};
pub enum IFEXIT {
    Next,
    Exit,
}
//计划将它设置成一个入口
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = AppSub::default();
    let informations = utils::start();
    if !informations.is_empty() {
        app.messages = informations
            .iter()
            .map(|amessage| spider::remove_quotation(amessage.ps.clone()))
            .collect();
        app.stateoflist = true;
        app.state.select(Some(0));
        app.informations = informations;
    }
    app.settings_input[0] = utils::start_v2core();
    loop {
        // here ,need with different tab ,use different draw funcitons
        //terminal.draw(|f| render::ui(f, &mut app))?;
        if let IFEXIT::Exit = app.run_app_local(terminal)? {
            return Ok(());
        }
    }
}
