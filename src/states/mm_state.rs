use crate::{
    configs::Locale,
    entity::label::*,
    utils::{constants::*, state::*, states::*},
};

use macroquad::prelude::*;

pub struct MainMenuState {
    label: Label,
    cursor: AnimatedLabel,
    version: Label,
    locale: Locale,
}

impl MainMenuState {
    pub fn new(locale: Locale) -> Self {
        Self {
            label: Label::new("Typix".to_string(), GOLOS_BOLD),
            cursor: AnimatedLabel::new(vec!["".to_owned(), "_".to_owned()], 0.5, GOLOS_BOLD),
            version: Label::new("Ver: ".to_string() + VERSION, GOLOS_REGULAR),
            locale,
        }
    }

    fn draw_ui(&mut self, dt: f32, stmc: &mut StateMachineController<TypixState>) {
        // Draw header
        self.label.draw(
            screen_width() / 2. - self.label.get_dimensions().width / 2.,
            self.label.get_dimensions().height * 1.5,
        );
        self.cursor.draw(
            dt,
            screen_width() / 2. - self.label.get_dimensions().width / 2.
                + self.label.get_dimensions().width,
            self.label.get_dimensions().height * 1.5,
        );

        // Draw version
        self.version.draw(
            10.,
            screen_height() - self.version.get_dimensions().height - 10.,
        );

        egui_macroquad::ui(|ctx| {
            let mut font = egui::FontDefinitions::default();
            font.font_data.insert(
                "GOLOS_REGULAR".to_owned(),
                egui::FontData::from_static(GOLOS_REGULAR).tweak(egui::FontTweak {
                    scale: 2.,
                    ..Default::default()
                }),
            );
            font.families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "GOLOS_REGULAR".to_owned());
            ctx.set_fonts(font);

            let mm_btn_width = screen_width() / 2.5;
            let mm_btn_height = screen_height() / 10.;

            egui::Area::new("mm_area")
                .fixed_pos(egui::pos2(
                    screen_width() / 2. - mm_btn_width / 2.,
                    screen_height() / 2. - mm_btn_height / 2.,
                ))
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .rounding(egui::Rounding::same(15.))
                        .fill(egui::Color32::BLACK)
                        .margin(egui::style::Margin::same(15.))
                        .show(ui, |ui| {
                            if ui
                                .add_sized(
                                    [mm_btn_width, mm_btn_height],
                                    egui::Button::new(&self.locale.locale["play"]),
                                )
                                .clicked()
                            {
                                stmc.set_state(TypixState::LvlSelect);
                            };
                            ui.add_space(5.);
                            if ui
                                .add_sized(
                                    [mm_btn_width, mm_btn_height],
                                    egui::Button::new(&self.locale.locale["settings"]),
                                )
                                .clicked()
                            {
                                stmc.set_state(TypixState::Settings);
                            };
                            ui.add_space(5.);
                            if ui
                                .add_sized(
                                    [mm_btn_width, mm_btn_height],
                                    egui::Button::new(&self.locale.locale["exit"]),
                                )
                                .clicked()
                            {
                                stmc.send_exit();
                            };
                        });
                });
        });

        egui_macroquad::draw();
    }
}

impl State<TypixState> for MainMenuState {
    fn load(&mut self) {
        self.label.set_font_size(120);
        self.cursor.set_font_size(120);
    }

    fn key_handler(&mut self, stmc: &mut StateMachineController<TypixState>) {}

    fn draw(&mut self, dt: f32, stmc: &mut StateMachineController<TypixState>) {
        self.draw_ui(dt, stmc);
    }
}
