use macroquad::prelude::*;

use crate::{
    configs::{get_list, Layout, Locale},
    states::*,
    utils::{state::StateMachine, states::TypixState},
};

mod configs;
mod entity;
mod states;
mod utils;

fn handle_keys() {
    if is_key_pressed(KeyCode::F11) {}
}

fn load_conf() -> Conf {
    let mut path = dirs::config_dir().unwrap();
    path.push("typix/settings.json");

    let conf = match configs::Settings::load(&path) {
        Ok(c) => c,
        Err(_) => {
            let mut p = path.clone();
            p.pop();
            std::fs::create_dir_all(&p).unwrap();
            let mut c = configs::Settings::default();
            p.push("settings.json");
            c.set_path(&p);
            c.save().unwrap();
            c
        }
    };
    Conf {
        window_title: "Typix".to_owned(),
        window_width: conf.window_width,
        window_height: conf.window_height,
        fullscreen: conf.fullscreen,
        sample_count: conf.samples,
        ..Default::default()
    }
}

#[macroquad::main(load_conf)]
async fn main() {
    let mut path = dirs::config_dir().unwrap();
    path.push("typix/configs/");
    get_list(&path).unwrap();

    let mut path = dirs::config_dir().unwrap();
    path.push("typix/settings.json");

    let settings = match configs::Settings::load(&path) {
        Ok(c) => c,
        Err(_) => {
            let mut p = path.clone();
            p.pop();
            std::fs::create_dir_all(&p).unwrap();
            let mut c = configs::Settings::default();
            p.push("settings.json");
            c.set_path(&p);
            c.save().unwrap();
            c
        }
    };

    let layout_conf = Layout::load("en.json").unwrap();
    let locale_conf = Locale::load(&format!("{}{}", settings.locale, ".json")).unwrap();

    let mut state_machine = StateMachine::<TypixState>::new();

    state_machine.add_state(TypixState::Main, MainMenuState::new(locale_conf.clone()));
    state_machine.add_state(
        TypixState::LvlSelect,
        LvlSelectState::new(locale_conf.clone()),
    );
    state_machine.add_state(
        TypixState::DropGame,
        DropGameState::new(layout_conf, locale_conf.clone()),
    );
    state_machine.add_state(
        TypixState::Settings,
        SettingsState::new(locale_conf.clone()),
    );

    state_machine.set_state(TypixState::Main);

    while state_machine.wait_exit_signal() {
        clear_background(Color::from_rgba(31, 31, 31, 255));

        state_machine.handle_keys();
        state_machine.draw(get_frame_time());

        next_frame().await
    }
}
