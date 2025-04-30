use std::{thread::sleep, time::Duration};

use bevy::prelude::*;
use bevy_settings::{Deserialize, PersistSetting, PersistSettings, Serialize};

#[derive(Resource, Default, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
struct Settings {
    master_volume: f64,
    custom_cursor: bool,
}

#[derive(Resource, Default, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
struct PlayerProfile {
    highscore: f64,
    deaths: usize,
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
            "The name of the game settings",
        ))
        .add_plugins(bevy_settings::SettingsPlugin::<PlayerProfile>::new(
            "My awesome game studio",
            "The name of the game profile",
        ))
        .add_systems(Update, (read_settings, persist_profile_setting))
        .run();
}

/// Resources from Settings are available as normal
fn read_settings(profile: Res<PlayerProfile>) {
    println!("Highscore {:?}", profile.highscore);
    sleep(Duration::from_millis(500));
}

/// this will just persist the settings file for a certain [`Resource`]
fn persist_profile_setting(
    mut profile: ResMut<PlayerProfile>,
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<PersistSetting<PlayerProfile>>,
    mut writer_all: EventWriter<PersistSettings>,
) {
    if keys.just_pressed(KeyCode::KeyS) {
        profile.highscore += 1.0;
        println!("Persisting Config {:?}", profile.into_inner());
        writer.write(PersistSetting::default());
    }
    if keys.just_pressed(KeyCode::KeyA) {
        println!("Persisting All Config");
        writer_all.write(PersistSettings);
    }
}
