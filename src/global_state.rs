
use ratatui::widgets::ListState;

pub enum InputMode {
    Normal,
    Insert
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }

    pub fn select_tab(&mut self, title: &str) {
        if let Some(index) = self.titles.iter().position(|t| *t == title) {
            self.index = index;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut sl = StatefulList {
            state: ListState::default(),
            items,
        };
        sl.state.select(Some(0));
        sl
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
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
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct State<'a> {
    pub blocks: StatefulList<String>,
    pub block_info: StatefulList<(String, String)>,
    pub focus: StatefulList<String>,
    pub input_buffer: String,
    pub input_mode: InputMode,
    pub rpc_endpoint: String,
    pub rpc_list: StatefulList<(String, String)>,
    pub rpc_list_popup: bool,
    pub rpc_selected: String,
    pub search_popup: bool,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub tabs_current: u16,
    pub tx_info: StatefulList<(String, String)>,
}

impl State<'_> {
    pub fn new() -> Self {
        Self {
            blocks: StatefulList::with_items(vec![]),
            block_info: StatefulList::with_items(vec![]),
            focus: StatefulList::with_items(vec!["last_blocks".to_string(), "block_info".to_string()]),
            input_buffer: String::new(),
            input_mode: InputMode::Normal,
            rpc_endpoint: std::env::var("ETH_RPC_ENDPOINT").unwrap(),
            rpc_list: StatefulList::with_items(vec![
                ("ETH_RPC_ENDPOINT".to_string(), "Ethereum".to_string()),
                ("GNOSIS_RPC_ENDPOINT".to_string(), "Gnosis Chain".to_string()),
                ("CUSTOM_RPC_ENDPOINT".to_string(), "Custom RPC Endpoint".to_string()),
            ]),
            rpc_list_popup: false,
            rpc_selected: "Ethereum".to_string(),
            search_popup: false,
            should_quit: false,
            tabs: TabsState::new(vec!["Blocks", "Transactions"]),
            tabs_current: 0,
            tx_info: StatefulList::with_items(vec![]),
        }
    }

    pub fn set_current_tab(&mut self, title: &str) {
        self.tabs.select_tab(title);
        self.tabs_current = self.tabs.index as u16;
    }

    pub fn set_input_mode(&mut self, input_mode: String) {
        match input_mode.as_str() {
            "normal" => self.input_mode = InputMode::Normal,
            "insert" => self.input_mode = InputMode::Insert,
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

