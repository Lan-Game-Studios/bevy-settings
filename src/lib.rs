extern crate directories;

use directories::ProjectDirs;

use bevy_app::{App, CoreStage, Plugin};
use bevy_ecs::{
    prelude::{EventReader, Resource},
    system::Res,
};

pub extern crate serde;
pub use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "cookie-td.toml";

pub struct SettingsPlugin<S: Resource + Copy + Serialize + Default + for<'a> Deserialize<'a>>(pub S);

impl<S: Resource + Copy + Serialize + Default + for<'a> Deserialize<'a>> SettingsPlugin<S> {
    pub fn resource() -> S {
        Self::load().unwrap_or_default()
    }

    fn load() -> Option<S> {
        let path = ProjectDirs::from("com", "TecBeast", "Cookie TD")?
            .config_dir()
            .join(CONFIG_FILE);
        if !path.exists() {
            return Some(S::default());
        }
        let settings_string = std::fs::read_to_string(path).ok()?;
        toml::from_str(&settings_string).ok()
    }

    fn persist(settings: Res<S>, mut reader: EventReader<PersistEvent>) {
        if reader.iter().len() > 0 {
            let project_dirs = ProjectDirs::from("com", "TecBeast", "Cookie TD").unwrap();
            let directory = project_dirs.config_dir();
            std::fs::create_dir_all(directory).expect("Couldn't write a configuration file");
            let path = (*directory).join(CONFIG_FILE);
            std::fs::write(path, toml::to_string(&*settings).unwrap()).unwrap();
        }
    }
}

impl<S: Resource + Copy + Serialize + Default + for<'a> Deserialize<'a>> Plugin
    for SettingsPlugin<S>
{
    fn build(&self, app: &mut App) {
        app.insert_resource(Self::resource())
            .add_event::<PersistEvent>()
            .add_system_to_stage(CoreStage::Last, SettingsPlugin::<S>::persist);
    }
}

pub struct PersistEvent;

