use crate::entity::label::Label;

use macroquad::prelude::*;

pub struct KeyDrop {
    pub x: f32,
    pub y: f32,
    pub label: Label,
}

impl KeyDrop {
    pub fn new(x: f32, y: f32, label: String, font: Font) -> Self {
        let mut label = Label::from_font(label, font);
        label.set_font_size(24);
        Self { x, y, label }
    }

    pub fn draw(&mut self, dt: f32, speed: f32) {
        self.y += speed * dt;
        self.label.draw(self.x, self.y);
    }

    pub fn check_fall(&self, y_max: f32) -> bool {
        if self.y >= y_max {
            return true;
        }

        false
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
