#[derive(Default, PartialEq, Debug)]
pub enum CurrentScreen {
    #[default]
    MainMenu,
    GameScene,
    EndScreen
}

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            should_quit: false,
            current_screen: CurrentScreen::MainMenu
        }
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
