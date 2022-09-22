use crate::app::App;
use crate::informations::appbar::AppBar;
//use crate::spider;
use crate::subscribe::appsub::AppSub;
use crate::utils;
use std::io;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::widgets::ListState;
use tui::Terminal;
#[derive(Clone, Copy)]
pub enum Page {
    SubScribe = 0,
    Information = 1,
}
// 这个做成tab的选择入口
pub enum IFEXIT {
    Next,
    Exit,
    Change(Page),
}

pub(crate) type MyBackend = CrosstermBackend<Stdout>;
//计划将它设置成一个入口
pub async fn run_app(terminal: &mut Terminal<MyBackend>) -> io::Result<()> {
    let mut appsub = AppSub::default();
    let informations = utils::start();
    if !informations.is_empty() {
        appsub.subs = informations
            .iter()
            .map(|ainformation| {
                ainformation
                    .iter()
                    .map(|message| message.ps.clone())
                    .collect()
            })
            .collect();
        appsub.stateoflist = true;
        let len = informations.len();
        appsub.state = vec![ListState::default(); len];
        //appsub.state[0].select(Some(0));
        appsub.informations = informations;
    }
    appsub.subscription = utils::get_subs();
    appsub.settings_input[0] = utils::start_v2core();
    let appbar = AppBar::new();
    let mut local_page = Page::SubScribe;
    let mut pages: Vec<Box<dyn App>> = vec![Box::new(appsub), Box::new(appbar)];

    loop {
        match pages[local_page as usize].run_app_local(terminal).await? {
            IFEXIT::Exit => return Ok(()),
            IFEXIT::Change(e) => local_page = e,
            IFEXIT::Next => {}
        }
    }
}
