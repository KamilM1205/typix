use crate::{
    configs::Locale,
    utils::{constants::*, state::*, states::*},
};

pub struct LvlSelectState {}

impl LvlSelectState {
    pub fn new(locale: Locale) -> Self {
        Self {}
    }
}

impl State<TypixState> for LvlSelectState {
    fn load(&mut self) {}

    fn key_handler(&mut self, stmc: &mut StateMachineController<TypixState>) {}

    fn draw(&mut self, dt: f32, stmc: &mut StateMachineController<TypixState>) {
        stmc.set_state(TypixState::DropGame);
    }
}
