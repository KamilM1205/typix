use macroquad::prelude::*;

pub struct AnimatedLabel {
    anim_frames: Vec<String>,
    frame: usize,
    delay: f32,
    cur_delay: f32,
    font: Font,
    font_size: u16,
    color: Color,
}

pub struct Label {
    text: String,
    font: Font,
    font_size: u16,
    color: Color,
}

impl AnimatedLabel {
    pub fn new(anim_frames: Vec<String>, delay: f32, font: &[u8]) -> Self {
        let f = load_ttf_font_from_bytes(font).unwrap();
        Self {
            anim_frames,
            frame: 0,
            delay,
            cur_delay: 0.,
            font: f,
            font_size: 16,
            color: WHITE,
        }
    }

    pub fn draw(&mut self, dt: f32, x: f32, y: f32) {
        if self.cur_delay >= self.delay {
            self.cur_delay = 0.;
            if self.frame != self.anim_frames.len() - 1 {
                self.frame += 1;
            } else {
                self.frame = 0;
            }
        }
        self.cur_delay += 1. * dt;
        draw_text_ex(
            &self.anim_frames[self.frame],
            x,
            y,
            TextParams {
                font_size: self.font_size,
                font: self.font,
                color: self.color,
                ..Default::default()
            },
        );
    }

    pub fn get_dimensions(&self) -> TextDimensions {
        measure_text(
            &self.anim_frames[self.frame],
            Some(self.font),
            self.font_size,
            1.,
        )
    }

    pub fn set_font_size(&mut self, font_size: u16) {
        self.font_size = font_size;
    }
}

impl Label {
    pub fn new(text: String, font_raw: &[u8]) -> Self {
        Self {
            text,
            font_size: 16,
            color: WHITE,
            font: load_ttf_font_from_bytes(font_raw).unwrap(),
        }
    }

    pub fn from_font(text: String, font: Font) -> Self {
        Self {
            text,
            font_size: 16,
            color: WHITE,
            font,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn get_dimensions(&self) -> TextDimensions {
        measure_text(&self.text, Some(self.font), self.font_size, 1.)
    }

    pub fn set_font_size(&mut self, font_size: u16) {
        self.font_size = font_size;
    }

    pub fn draw(&mut self, x: f32, y: f32) {
        draw_text_ex(
            &self.text,
            x,
            y,
            TextParams {
                font: self.font,
                font_size: self.font_size,
                color: self.color,
                ..Default::default()
            },
        );
    }
}
