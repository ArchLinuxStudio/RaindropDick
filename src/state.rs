use crate::app::App;
use crate::informations::appbar::AppBar;
use crate::spider;
use crate::subscribe::appsub::AppSub;
use crate::utils;
use std::io;
use tui::{backend::Backend, Terminal};

pub enum Page {
    SubScribe,
    Information,
}
// 这个做成tab的选择入口
pub enum IFEXIT {
    Next,
    Exit,
    Change(Page),
}
//计划将它设置成一个入口
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut appsub = AppSub::default();
    let informations = utils::start();
    if !informations.is_empty() {
        appsub.messages = informations
            .iter()
            .map(|amessage| spider::remove_quotation(amessage.ps.clone()))
            .collect();
        appsub.stateoflist = true;
        appsub.state.select(Some(0));
        appsub.informations = informations;
    }
    let mut appbar = AppBar::new();
    appsub.settings_input[0] = utils::start_v2core();
    let mut local_page = Page::SubScribe;
    loop {
        match local_page {
            Page::SubScribe => match appsub.run_app_local(terminal)? {
                IFEXIT::Exit => return Ok(()),
                IFEXIT::Change(e) => local_page = e,
                IFEXIT::Next => {}
            },
            Page::Information => match appbar.run_app_local(terminal)? {
                IFEXIT::Exit => return Ok(()),
                IFEXIT::Change(e) => local_page = e,
                IFEXIT::Next => {}
            },
        }
    }
}
