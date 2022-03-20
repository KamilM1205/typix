use crate::{
    configs::{Locale, Settings},
    entity::{label::Label, messages::TwoButtonMessage},
    utils::{
        constants::{GOLOS_BOLD, GOLOS_REGULAR, SAMPLES},
        state::*,
        states::TypixState,
    },
};

use macroquad::prelude::*;

pub struct SettingsState {
    locale: Locale,
    label: Label,
    font: egui::FontDefinitions,
    settings: Settings,
    exit_message: TwoButtonMessage,
    require_restart: bool,
    changed: bool,
}

impl SettingsState {
    pub fn new(locale: Locale) -> Self {
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

        let mut path = dirs::config_dir().unwrap();
        path.push("typix/settings.json");

        let settings = match Settings::load(&path) {
            Ok(s) => s,
            Err(_) => {
                let mut p = path.clone();
                p.pop();
                std::fs::create_dir_all(&p).unwrap();
                let c = Settings::default();
                c.save().unwrap();
                c
            }
        };

        Self {
            label: Label::new((&locale.locale["settings"]).into(), GOLOS_BOLD),
            font,
            settings,
            exit_message: TwoButtonMessage::new(
                &locale.locale["s_message_ok"],
                &locale.locale["s_message_cancel"],
                &locale.locale["s_message_title"],
                &locale.locale["s_message_msg"],
            ),
            locale,
            require_restart: false,
            changed: false,
        }
    }

    fn draw_ui(&mut self) {
        let dimensions = self.label.get_dimensions();
        self.label.draw(
            screen_width() / 2. - dimensions.width / 2.,
            dimensions.height,
        );

        egui_macroquad::ui(|ctx| {
            let (width, height) = (screen_width() / 2., screen_height() / 2.);
            ctx.set_fonts(self.font.clone());

            egui::Area::new("settings_area")
                .fixed_pos(egui::pos2(
                    screen_width() / 2. - width / 2.,
                    screen_height() / 2. - height / 2.,
                ))
                .order(egui::Order::Background)
                .interactable(!self.exit_message.is_showed())
                .show(ctx, |ui| {
                    egui::Frame::none()
                        .rounding(egui::Rounding::same(15.))
                        .fill(egui::Color32::BLACK)
                        .margin(egui::style::Margin::same(15.))
                        .show(ui, |ui| {
                            egui::Resize::default()
                                .fixed_size(egui::Vec2::new(width, height))
                                .resizable(false)
                                .show(ui, |ui| {
                                    egui::ScrollArea::vertical().show(ui, |ui| {
                                        ui.with_layout(
                                            egui::Layout::top_down_justified(egui::Align::LEFT),
                                            |ui| {
                                                ui.set_min_width(width);
                                                ui.collapsing(
                                                    self.locale.locale["s_game"].clone(),
                                                    |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label(
                                                                self.locale.locale
                                                                    ["s_game_language"]
                                                                    .clone(),
                                                            );
                                                            egui::ComboBox::from_id_source(
                                                                "s_game_lsel",
                                                            )
                                                            .selected_text(&self.settings.locale)
                                                            .show_ui(ui, |ui| {
                                                                for i in &self.locale.list {
                                                                    if ui
                                                                        .selectable_value(
                                                                            &mut self
                                                                                .settings
                                                                                .locale,
                                                                            i.clone(),
                                                                            i,
                                                                        )
                                                                        .clicked()
                                                                    {
                                                                        self.changed = true;
                                                                    }
                                                                }
                                                            });
                                                        });
                                                        ui.horizontal(|ui| {
                                                            ui.label(
                                                                &self.locale.locale
                                                                    ["s_game_show_fps"],
                                                            );
                                                            if ui
                                                                .checkbox(
                                                                    &mut self.settings.show_fps,
                                                                    "",
                                                                )
                                                                .changed()
                                                            {
                                                                self.changed = true;
                                                                self.require_restart = true;
                                                            }
                                                        });
                                                    },
                                                );
                                                ui.collapsing(
                                                    &self.locale.locale["s_video"],
                                                    |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.label(
                                                                &self.locale.locale
                                                                    ["s_video_fscreen"],
                                                            );
                                                            if ui
                                                                .checkbox(
                                                                    &mut self.settings.fullscreen,
                                                                    "",
                                                                )
                                                                .changed()
                                                            {
                                                                self.changed = true;
                                                                self.require_restart = true;
                                                            }
                                                        });
                                                        ui.horizontal(|ui| {
                                                            ui.label(
                                                                &self.locale.locale
                                                                    ["s_video_samples"],
                                                            );

                                                            let msaa_selected =
                                                                match self.settings.samples {
                                                                    0 => self.locale.locale
                                                                        ["s_video_samples_off"]
                                                                        .clone(),
                                                                    _ => self
                                                                        .settings
                                                                        .samples
                                                                        .to_string(),
                                                                };

                                                            egui::ComboBox::from_id_source(
                                                                "msaa_select",
                                                            )
                                                            .selected_text(msaa_selected)
                                                            .show_ui(ui, |ui| {
                                                                for (i, v) in
                                                                    SAMPLES.iter().enumerate()
                                                                {
                                                                    let v = if i == 0 {
                                                                        self.locale.locale
                                                                            ["s_video_samples_off"]
                                                                            .clone()
                                                                    } else {
                                                                        v.to_string()
                                                                    };

                                                                    if ui
                                                                        .selectable_value(
                                                                            &mut self
                                                                                .settings
                                                                                .samples,
                                                                            SAMPLES[i],
                                                                            format!("{}", v),
                                                                        )
                                                                        .clicked()
                                                                    {
                                                                        self.changed = true;
                                                                        self.require_restart = true;
                                                                    }
                                                                }
                                                            });
                                                        });
                                                    },
                                                );
                                                ui.collapsing(
                                                    self.locale.locale["s_audio"].clone(),
                                                    |ui| {},
                                                );
                                                ui.collapsing(
                                                    self.locale.locale["s_theme"].clone(),
                                                    |ui| {},
                                                );
                                            },
                                        );
                                    });
                                });
                        });
                });

            self.exit_message.draw(ctx, |msg| {
                msg.hide();
                self.settings.save().unwrap();
                self.changed = false;
                self.require_restart = false;
            });
        });

        egui_macroquad::draw();
    }
}

impl State<TypixState> for SettingsState {
    fn load(&mut self) {
        self.label.set_font_size(120);
    }

    fn key_handler(self: &mut Self, stmc: &mut StateMachineController<TypixState>) {
        if is_key_pressed(KeyCode::Escape) {
            if !self.require_restart && !self.changed {
                stmc.set_state(TypixState::Main);
                return;
            } else {
                self.exit_message.show();
            }
        }
    }

    fn draw(&mut self, dt: f32, stmc: &mut StateMachineController<TypixState>) {
        self.draw_ui();
    }
}
