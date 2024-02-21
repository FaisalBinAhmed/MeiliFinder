
#[derive(PartialEq)] // need this to do binary comparison
pub enum AppTabs {
    HomeTab,
    StationTab,
}

#[derive(PartialEq)]
pub enum AppMode {
    Normal,
    Search,
}

pub struct App {
    pub selected_tab: AppTabs,
    pub should_quit: bool,

    pub should_redraw: bool,
    pub status: String,
    pub app_mode: AppMode,
}

impl App {
    pub async fn new() -> Self {
        Self {
            selected_tab: AppTabs::HomeTab,
            should_quit: false,
            should_redraw: true,
            status: "Loading stations...".to_string(),
            app_mode: AppMode::Normal,
        }
    }
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
    
}
