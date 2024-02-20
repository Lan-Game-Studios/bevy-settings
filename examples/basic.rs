use bevy::prelude::*;
use bevy_settings::{Deserialize, PersistSettings, Serialize};

#[derive(Resource, Default, Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(crate = "bevy_settings::serde")]
struct Settings {
    master_volume: f64,
    custom_cursor: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_settings::SettingsPlugin::<Settings>::new(
            "My awesome game studio",
            "The name of the game",
        ))
        .add_systems(Update, (read_settings, persist_settings))
        .run();
}

fn read_settings(settings: Res<Settings>) {
    println!("Master Volume {:?}", settings.master_volume);
}

fn persist_settings(
    settings: Res<Settings>,
    keys: Res<ButtonInput<KeyCode>>,
    mut writer: EventWriter<PersistSettings>,
) {
    if keys.just_pressed(KeyCode::KeyS) {
        println!("Persisting Config {:?}", settings.into_inner());
        writer.send(PersistSettings);
    }
}
