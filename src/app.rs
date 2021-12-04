use tui::widgets::ListState;
use crate::spider;
pub enum InputMode {
    Normal,
    Editing,
    Select,
    Popup,
    PopupEdit,
    SubscriptView,
}

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: String,
    pub settings_input: Vec<String>,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<String>,
    pub state: ListState,
    pub index_subscription: ListState,
    pub index_settings: usize,
    pub stateoflist: bool,
    pub show_popup: bool,
    pub informations: Vec<spider::Information>,
    pub subscription : Vec<String>,
}
impl App {
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.messages.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.messages.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
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
impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            settings_input: vec![String::new(), String::new()],
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            state: ListState::default(),
            index_subscription: ListState::default(),
            index_settings: 0,
            stateoflist: false,
            show_popup: false,
            informations: Vec::new(),
            subscription: Vec::new(),
        }
    }
}

