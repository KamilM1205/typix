use crate::{entity::label::Label, utils::constants::GOLOS_REGULAR};

use macroquad::time::get_fps;

pub struct FPS {
    label: Label,
    prefix: String,
    show: bool,
}

impl Default for FPS {
    fn default() -> Self {
        Self {
            label: Label::new(get_fps().to_string(), GOLOS_REGULAR),
            prefix: String::new(),
            show: false,
        }
    }
}

impl FPS {
    fn new(prefix: impl Into<String>) -> Self {
        Self {
            label: Label::new(get_fps().to_string(), GOLOS_REGULAR),
            prefix: prefix.into(),
            show: false,
        }
    }

    pub fn set_show(&mut self, show: bool) {
        self.show = show;
    }

    pub fn draw(&mut self) {
        if !self.show {
            return;
        }

        self.label.set_text(get_fps().to_string());
        let label_dim = self.label.get_dimensions();
        self.label.draw(0., label_dim.height);
    }
}
