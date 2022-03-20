use crate::{
    configs::Locale,
    entity::label::Label,
    utils::{constants::*, state::*, states::*},
};

use macroquad::prelude::*;

pub struct LvlSelectState {
    label: Label,
    locale: Locale,
}

impl LvlSelectState {
    pub fn new(locale: Locale) -> Self {
        Self {
            label: Label::new((&locale.locale["lvlselect_label"]).into(), GOLOS_BOLD),
            locale,
        }
    }

    pub fn draw_ui(&mut self, stmc: &mut StateMachineController<TypixState>) {
        egui_macroquad::ui(|ctx| {
            egui::Area::new("lvlselect_area")
                .default_pos(egui::pos2(0., 0.))
                .show(ctx, |ui| {
                    egui::Frame::default()
                        .fill(egui::Color32::BLACK)
                        .margin(egui::style::Margin::same(15.))
                        .show(ui, |ui| {});
                });
        });

        egui_macroquad::draw();
    }
}

impl State<TypixState> for LvlSelectState {
    fn load(&mut self) {
        self.label.set_font_size(120);
    }

    fn key_handler(&mut self, stmc: &mut StateMachineController<TypixState>) {
        if is_key_pressed(KeyCode::Escape) {
            stmc.set_state(TypixState::Main);
        }
    }

    fn draw(&mut self, dt: f32, stmc: &mut StateMachineController<TypixState>) {
        //self.draw_ui(stmc);
        stmc.set_state(TypixState::DropGame);
    }
}
