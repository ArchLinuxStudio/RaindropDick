use super::app::*;
use super::render::ui;
use super::state::subscribe_state;
use crate::spider;
use crate::state::{MyBackend, IFEXIT};
use std::io;
use tokio::sync::mpsc::Receiver;
use tui::widgets::ListState;
use tui::Terminal;
pub(super) enum InputMode {
    Normal,
    Editing,
    Select,
    Popup,
    PopupEdit,
    SubscriptView,
}
/// App holds the state of the application
pub struct AppSub {
    /// Current value of the input box
    // search bar
    pub input: String,
    // settings , include coresetting and subscribe setting
    pub settings_input: Vec<String>,
    /// Current input mode
    pub(super) input_mode: InputMode,
    // History of recorded subs
    // subs's names
    pub subs: Vec<Vec<String>>,
    pub subsindex: usize,
    pub state: Vec<ListState>,
    pub index_subscription: ListState,
    pub index_settings: usize,
    pub stateoflist: bool,
    pub show_popup: bool,
    // subscribes's information
    pub informations: Vec<Vec<spider::Information>>,
    pub subscription: Vec<String>,
    pub receiver: Option<Receiver<reqwest::Result<Vec<Vec<String>>>>>,
    pub popinfomation: String,
}
impl AppSub {
    pub fn next(&mut self) {
        let i = match self.state[self.subsindex].selected() {
            Some(i) => {
                if i >= self.subs[self.subsindex].len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state[self.subsindex].select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state[self.subsindex].selected() {
            Some(i) => {
                if i == 0 {
                    self.subs[self.subsindex].len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state[self.subsindex].select(Some(i));
    }
    pub fn left(&mut self) {
        if self.subsindex == 0 {
            self.subsindex = self.state.len() - 1;
        } else {
            self.subsindex -= 1;
        }
    }
    pub fn right(&mut self) {
        if self.subsindex >= self.state.len() - 1 {
            self.subsindex = 0;
        } else {
            self.subsindex += 1;
        }
    }
    pub fn unselect(&mut self) {
        self.state[self.subsindex].select(None);
    }
    pub fn next_sub(&mut self) {
        let i = match self.index_subscription.selected() {
            Some(i) => {
                if i >= self.subscription.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.index_subscription.select(Some(i));
        //self.index = Some(i);
    }

    pub fn previous_sub(&mut self) {
        let i = match self.index_subscription.selected() {
            Some(i) => {
                if i == 0 {
                    self.subscription.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.index_subscription.select(Some(i));
        //self.index = Some(i);
    }

    pub fn unselect_sub(&mut self) {
        self.index_subscription.select(None);
    }
}
use async_trait::async_trait;
#[async_trait]
impl App for AppSub {
    async fn run_app_local(&mut self, terminal: &mut Terminal<MyBackend>) -> io::Result<IFEXIT> {
        terminal.draw(|f| ui(f, self))?;
        subscribe_state(self).await
    }
}
impl Default for AppSub {
    fn default() -> AppSub {
        AppSub {
            input: String::new(),
            settings_input: vec![String::new(), String::new()],
            input_mode: InputMode::Normal,
            subs: vec![vec![]],
            subsindex: 0,
            state: vec![ListState::default()],
            index_subscription: ListState::default(),
            index_settings: 0,
            stateoflist: false,
            show_popup: false,
            informations: vec![vec![]],
            subscription: Vec::new(),
            receiver: None,
            popinfomation: "Settings, e to edit, s to save".to_string(),
        }
    }
}
