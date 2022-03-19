use macroquad::prelude::*;

pub struct OneButtonMessage {
    ok: String,
    title: String,
    message: String,
    show: bool,
}

impl OneButtonMessage {
    pub fn new(ok: impl Into<String>, title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            ok: ok.into(),
            title: title.into(),
            message: message.into(),
            show: false,
        }
    }

    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn hide(&mut self) {
        self.show = false;
    }

    pub fn is_showed(&self) -> bool {
        self.show
    }

    pub fn draw(&mut self, ctx: &egui::Context) {
        if !self.show {
            return
        }
    }
}

pub struct TwoButtonMessage {
    ok: String,
    cancel: String,
    title: String,
    message: String,
    show: bool,
}

impl TwoButtonMessage {
    pub fn new(
        ok: impl Into<String>,
        cancel: impl Into<String>,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            ok: ok.into(),
            cancel: cancel.into(),
            title: title.into(),
            message: message.into(),
            show: false,
        }
    }

    pub fn show(&mut self) {
        self.show = true;
    }

    pub fn hide(&mut self) {
        self.show = false;
    }

    pub fn is_showed(&self) -> bool {
        self.show
    }

    pub fn draw(&mut self, ctx: &egui::Context, ok_event: impl FnOnce(&mut TwoButtonMessage)) {
        if self.show == false {
            return;
        }

        let (width, height) = (screen_width() / 3., screen_height() / 10.);

        egui::Area::new("message_background")
            .fixed_pos(egui::pos2(0., 0.))
            .order(egui::Order::Middle)
            .show(ctx, |ui| {
                egui::Frame::default()
                    .fill(egui::Color32::from_rgba_premultiplied(20, 20, 20, 150))
                    .show(ui, |ui| {
                        egui::Resize::default()
                            .fixed_size(egui::vec2(screen_width(), screen_height()))
                            .show(ui, |_ui| {});
                    });
            });

        egui::Area::new("message_area")
            .fixed_pos(egui::pos2(
                screen_width() / 2. - width / 2.,
                screen_height() / 2. - height / 2.,
            ))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::default()
                    .rounding(egui::Rounding::same(15.))
                    .fill(egui::Color32::BLACK)
                    .margin(egui::style::Margin::same(15.))
                    .show(ui, |ui| {
                        egui::Resize::default()
                            .fixed_size(egui::Vec2::new(width, height))
                            .show(ui, |ui| {
                                ui.with_layout(
                                    egui::Layout::top_down_justified(egui::Align::Center),
                                    |ui| {
                                        ui.label(&self.title);
                                        ui.separator();
                                        ui.add_space(5.);
                                        ui.label(&self.message);
                                        ui.add_space(5.);
                                        ui.horizontal(|ui| {
                                            if ui.button(&self.cancel).clicked() {
                                                self.show = false;
                                            }

                                            if ui.button(&self.ok).clicked() {
                                                ok_event(self);
                                            }
                                        });
                                    },
                                );
                            });
                    });
            });
    }
}
