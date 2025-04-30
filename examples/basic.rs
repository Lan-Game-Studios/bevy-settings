use std::{thread::sleep, time::Duration};

use bevy::prelude::*;
use bevy_settings::{Deserialize, PersistSettings, Serialize};

#[derive(Resource, Default, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
struct Settings {
    master_volume: f64,
    custom_cursor: bool,
    something: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(bevy_log::LogPlugin {
            filter: "warn,bevy_settings=debug".into(),
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(bevy_settings::SettingsPlugin::<Settings>::new(
            "My awesome game studio",
            "The name of the game",
        ))
        .add_systems(Update, (read_settings, persist_settings))
        .run();
}

fn read_settings(settings: Res<Settings>) {
    println!("Master Volume {:?}", settings.master_volume);
    sleep(Duration::from_millis(500));
}

fn persist_settings(
    mut settings: ResMut<Settings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<PersistSettings>,
) {
    if keys.just_pressed(KeyCode::KeyS) {
        settings.master_volume += 1.0;
        settings.something = 10_000;
        println!("Persisting Config {:?}", settings.into_inner());
        writer.write(PersistSettings);
    }
}
