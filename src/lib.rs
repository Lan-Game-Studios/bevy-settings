extern crate directories;

use std::{marker::PhantomData, path::PathBuf};

use directories::ProjectDirs;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    prelude::{Event, EventReader, Resource},
    system::Res,
};

pub extern crate serde;
pub use serde::{Deserialize, Serialize};

#[derive(Event)]
pub struct PersistSettings;

pub struct SettingsPlugin<S: Resource + Copy + Serialize + Default + for<'a> Deserialize<'a>> {
    domain: String,
    company: String,
    project: String,
    settings: PhantomData<S>,
}

#[derive(Resource, Debug)]
pub struct SettingsConfig {
    directory: PathBuf,
    path: PathBuf,
}

impl<S: Resource + Copy + Serialize + Default + for<'a> Deserialize<'a>> SettingsPlugin<S> {
    pub fn new(project: impl Into<String>, company: impl Into<String>) -> Self {
        Self {
            domain: "com".into(),
            company: company.into(),
            project: project.into(),
            settings: PhantomData::<S>,
        }
    }

    pub fn resource(&self) -> S {
        self.load().unwrap_or_default()
    }

    fn load(&self) -> Option<S> {
        let path = self.path();
        if !path.exists() {
            return Some(S::default());
        }
        let settings_string = std::fs::read_to_string(path).ok()?;
        toml::from_str(&settings_string).ok()
    }

    fn path(&self) -> PathBuf {
        ProjectDirs::from(&self.domain, &self.company, &self.project)
            .expect("Couldn't build settings path")
            .config_dir()
            .join(format!("{}.toml", self.project))
    }

    fn settings_directory(&self) -> PathBuf {
        ProjectDirs::from(&self.domain, &self.company, &self.project)
            .expect("Couldn't find a folder to store the settings")
            .config_dir()
            .to_path_buf()
    }

    fn persist(
        settings: Res<S>,
        config: Res<SettingsConfig>,
        reader: EventReader<PersistSettings>,
    ) {
        if !reader.is_empty() {
            std::fs::create_dir_all(config.directory.clone())
                .expect("Couldn't create the folders for the settings file");
            std::fs::write(
                config.path.clone(),
                toml::to_string(&*settings).expect("Couldn't serialize the settings to toml"),
            )
            .expect("couldn't persist the settings while trying to write the string to disk");
        }
    }
}

impl<S: Resource + Copy + Serialize + Default + for<'a> Deserialize<'a>> Plugin
    for SettingsPlugin<S>
{
    fn build(&self, app: &mut App) {
        app.insert_resource(self.resource())
            .insert_resource(SettingsConfig {
                directory: self.settings_directory(),
                path: self.path(),
            })
            .add_event::<PersistSettings>()
            .add_systems(Update, SettingsPlugin::<S>::persist);
    }
}
