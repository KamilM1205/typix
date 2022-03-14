use crate::{
    configs::{Layout, Locale},
    entity::{key_drop::KeyDrop, label::Label},
    utils::{constants::*, state::*, states::*},
};

use macroquad::prelude::*;

pub struct DropGameState {
    // Start state
    label_start: Label,
    // Pause state
    label_pause: Label,
    label_pause_help: Label,
    // Game state
    key_drops: Vec<KeyDrop>,
    score_label: Label,
    score: u32,
    speed: f32,
    last_time: f64,
    key_font: Font,
    layout: Layout,
    state: DropGameStates,
    locale: Locale,
}

impl DropGameState {
    pub fn new(layout: Layout, locale: Locale) -> Self {
        Self {
            // Start state
            label_start: Label::new((&locale.locale["dg_start"]).into(), GOLOS_BOLD),
            // Pause state
            label_pause: Label::new((&locale.locale["dg_pause"]).into(), GOLOS_BOLD),
            label_pause_help: Label::new((&locale.locale["dg_pause_help"]).into(), GOLOS_REGULAR),
            // Game state
            key_drops: Vec::new(),
            score_label: Label::new((&locale.locale["dg_score"]).into(), GOLOS_BOLD),
            score: 0,
            speed: 100.,
            last_time: 0.,
            key_font: load_ttf_font_from_bytes(GOLOS_BOLD).unwrap(),
            layout,
            state: DropGameStates::Start,
            locale,
        }
    }

    fn draw_start(&mut self, screen: Rect) {
        let dimensions = self.label_start.get_dimensions();
        self.label_start.draw(
            screen.x + screen.w / 2. - dimensions.width / 2.,
            screen.y + screen.h / 2. - dimensions.height / 2.,
        );
    }

    fn draw_game(&mut self, dt: f32, screen: Rect) {
        if get_time() - self.last_time > 0.1 {
            self.last_time = get_time();
            let char_indx = rand::gen_range(0, self.layout.litterals.len() as usize);
            let mut kd = KeyDrop::new(
                100.,
                0.,
                String::from(self.layout.litterals.as_bytes()[char_indx] as char),
                self.key_font.clone(),
            );
            let dimensions = kd.label.get_dimensions();
            kd.set_pos(
                rand::gen_range(screen.x, screen.w - dimensions.width),
                screen.y + dimensions.height / 2.,
            );
            self.key_drops.push(kd);
        }

        for k in &mut self.key_drops {
            k.draw(dt, self.speed);
        }

        let key = self.key_drops.get(0);
        if let Some(key) = key {
            let dimensions = key.label.get_dimensions();
            if key.check_fall(screen.h, dimensions.height) {
                self.key_drops.remove(0);
            }
        }
    }

    fn draw_pause(&mut self, screen: Rect) {
        let dimensions = self.label_pause.get_dimensions();
        let center = (screen.x + screen.w / 2., screen.y + screen.h / 2.);

        self.label_pause.draw(
            center.0 - dimensions.width / 2.,
            center.1 - dimensions.height / 2.,
        );

        let help_dimensions = self.label_pause_help.get_dimensions();

        self.label_pause_help.draw(
            center.0 - help_dimensions.width / 2.,
            center.1 + dimensions.height / 2. - help_dimensions.height / 2.,
        );
    }

    fn draw_stat(&mut self, ctx: &egui::Context, screen: Rect) {
        egui::Area::new("dropgame_stat")
            .fixed_pos(egui::pos2(
                screen.x,
                screen.h + screen_width() / 95.,
            ))
            .show(ctx, |ui| {
                egui::Frame::default()
                    .rounding(egui::Rounding::same(15.))
                    .fill(egui::Color32::BLACK)
                    .margin(egui::style::Margin::same(15.))
                    .show(ui, |ui| {
                        egui::Resize::default()
                            .fixed_size(egui::vec2(
                                screen_width() / 2. - 3.* screen_width() / 95.,
                                screen_height() - (screen.h + (screen_height() / 95.) * 6.),
                            ))
                            .show(ui, |ui| {
                                ui.label(&self.locale.locale["dg_statistics"]);
                                ui.separator();
                            });
                    });
            });
    }

    fn draw_game_setup(&mut self, ctx: &egui::Context, screen: Rect) {
        egui::Area::new("dropgame_setup")
            .fixed_pos(egui::pos2(
                screen_width() / 2. + screen_width() / 95.,
                screen.h + screen_width() / 95.,
            ))
            .show(ctx, |ui| {
                egui::Frame::default()
                    .rounding(egui::Rounding::same(15.))
                    .fill(egui::Color32::BLACK)
                    .margin(egui::style::Margin::same(15.))
                    .show(ui, |ui| {
                        egui::Resize::default()
                            .fixed_size(egui::vec2(
                                screen_width()/2. - (screen_width() - screen.w)*2.,
                                screen_height() - (screen.h + (screen_height() / 95.) * 6.),
                            ))
                            .show(ui, |ui| {
                                ui.label(&self.locale.locale["dg_settings"]);
                                ui.separator();
                            });
                    });
            });
    }

    fn draw_ui(&mut self, dt: f32) -> Rect {
        let sl_dimensions = self.score_label.get_dimensions();

        draw_text(&get_fps().to_string(), 20., 20., 15., WHITE);

        self.score_label.draw(
            screen_width() / 2. - sl_dimensions.width / 2.,
            sl_dimensions.height,
        );

        let sl_dimensions = self.score_label.get_dimensions();
        let mut game_screen_size = Rect::new(
            screen_width() / 90.,
            sl_dimensions.height * 2.,
            screen_width() - (screen_width() / 90.) * 2.,
            screen_height() / 1.5,
        );

        draw_rectangle_lines(
            game_screen_size.x,
            game_screen_size.y,
            game_screen_size.w,
            game_screen_size.h,
            3.,
            WHITE,
        );

        game_screen_size.h += sl_dimensions.height * 2.;

        egui_macroquad::ui(|ctx| {
            self.draw_stat(ctx, game_screen_size);
            self.draw_game_setup(ctx, game_screen_size);
        });

        egui_macroquad::draw();

        game_screen_size
    }
}

impl State<TypixState> for DropGameState {
    fn load(&mut self) {
        self.state = DropGameStates::Start;
        self.score_label.set_font_size(25);
        self.label_start.set_font_size(80);
        self.label_pause.set_font_size(80);
        self.label_pause_help.set_font_size(25);
    }

    fn key_handler(&mut self, stmc: &mut StateMachineController<TypixState>) {
        match self.state {
            DropGameStates::Start => {
                if is_key_pressed(KeyCode::Escape) {
                    stmc.set_state(TypixState::Main);
                }

                if is_key_pressed(KeyCode::Space) {
                    self.state = DropGameStates::Game;
                }
            }
            DropGameStates::Game => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = DropGameStates::Pause;
                }

                let ch = get_char_pressed();
                if let Some(ch) = ch {
                    let ch = ch.to_string();
                    for (i, k) in self.key_drops.iter().enumerate() {
                        if k.label.get_text() == ch {
                            self.key_drops.remove(i);
                            self.score += 1;
                            break;
                        }
                    }
                }
            }
            DropGameStates::Pause => {
                if is_key_pressed(KeyCode::Escape) {
                    self.state = DropGameStates::Start;
                    self.score = 0;
                    self.key_drops.clear();
                    self.last_time = 0.;
                }

                if is_key_pressed(KeyCode::Space) {
                    self.state = DropGameStates::Game;
                }
            }
            DropGameStates::GOver => {}
        }
    }

    fn draw(&mut self, dt: f32, stmc: &mut StateMachineController<TypixState>) {
        let screen = self.draw_ui(dt);

        self.score_label
            .set_text((&self.locale.locale["dg_score"]).to_string() + &self.score.to_string());

        match self.state {
            DropGameStates::Start => self.draw_start(screen),
            DropGameStates::Game => self.draw_game(dt, screen),
            DropGameStates::Pause => self.draw_pause(screen),
            DropGameStates::GOver => (),
        }
    }
}
