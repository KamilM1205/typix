#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum TypixState {
    Main,
    LvlSelect,
    DropGame,
    Settings,
}

pub enum DropGameStates {
    Start,
    Game,
    Pause,
    GOver,
}
