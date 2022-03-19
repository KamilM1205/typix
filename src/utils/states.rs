#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum TypixState {
    Main,
    LvlSelect,
    DropGame,
    Settings,
}

#[derive(Eq, PartialEq)]
pub enum DropGameStates {
    Start,
    Game,
    Pause,
    GOver,
}
