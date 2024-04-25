extern crate directories;

use std::{fmt::Debug, marker::PhantomData, path::PathBuf};

use directories::ProjectDirs;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    event::EventWriter,
    prelude::{Event, EventReader, Resource},
    schedule::{
        common_conditions::resource_changed,
        IntoSystemConfigs,
    },
    system::Res,
};
use bevy_log::prelude::debug;

pub extern crate serde;
pub use serde::{Deserialize, Serialize};

/// this is a blanked trait
pub trait Settingable: Resource + Clone + Serialize + Default + for<'a> Deserialize<'a> {}

/// this is a blanked implementation of [`Settingable`]
impl<S> Settingable for S where S: Resource + Clone + Serialize + Default + for<'a> Deserialize<'a> {}

/// This will persist all structs that
/// are added via the Plugin [`SettingsPlugin`]
#[derive(Event)]
pub struct PersistSettings;

/// This will persist structs S that
/// was added via the Plugin [`SettingsPlugin`]
#[derive(Event, Default)]
pub struct PersistSetting<S: Settingable>(PhantomData<S>);

pub struct SettingsPlugin<S: Settingable> {
    domain: String,
    company: String,
    project: String,
    settings: PhantomData<S>,
    autosave: bool,
}

#[derive(Resource, Debug)]
pub struct SettingsConfig<S: Settingable> {
    directory: PathBuf,
    path: PathBuf,
    settings: PhantomData<S>,
}

impl<S: Settingable> SettingsPlugin<S> {
    pub fn new(company: impl Into<String>, project: impl Into<String>) -> Self {
        Self {
            domain: "com".into(),
            company: company.into(),
            project: project.into(),
            settings: PhantomData::<S>,
            autosave: false,
        }
    }

    pub fn autosave(mut self) -> Self {
        self.autosave = true;
        self
    }

    pub fn resource(&self) -> S {
        self.load().unwrap_or_default()
    }

    fn load(&self) -> Option<S> {
        let path = self.path();
        if !path.exists() {
            return None;
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
        config: Res<SettingsConfig<S>>,
        reader_single: EventReader<PersistSetting<S>>,
        reader_all: EventReader<PersistSettings>,
    ) {
        debug!("System persist called");
        if !reader_single.is_empty() || !reader_all.is_empty() {
            std::fs::create_dir_all(config.directory.clone())
                .expect("Couldn't create the folders for the settings file");
            std::fs::write(
                config.path.clone(),
                toml::to_string(&*settings).expect("Couldn't serialize the settings to toml"),
            )
            .expect("couldn't persist the settings while trying to write the string to disk");
        }
    }

    fn trigger_autosave(mut writer: EventWriter<PersistSetting<S>>) {
        writer.send_default();
    }
}

impl<S: Settingable> Plugin for SettingsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.resource())
            .insert_resource(SettingsConfig {
                directory: self.settings_directory(),
                path: self.path(),
                settings: PhantomData::<S>,
            })
            .add_event::<PersistSettings>()
            .add_event::<PersistSetting<S>>()
            .add_systems(Update, SettingsPlugin::<S>::persist);
        if self.autosave {
            app.add_systems(
                Update,
                SettingsPlugin::<S>::trigger_autosave
                    .run_if(
                        resource_changed::<SettingsConfig<S>>
                    )
                    .before(SettingsPlugin::<S>::persist),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PersistSettings, SettingsPlugin};
    use bevy::prelude::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use serial_test::serial;

    use crate::PersistSetting;
    pub use crate::{Deserialize, Serialize};

    #[derive(Resource, Default, Serialize, Deserialize, Clone)]
    struct TestSetting1 {
        test: u32,
    }

    #[derive(Resource, Default, Serialize, Deserialize, Clone)]
    struct TestSetting2 {
        test: u32,
    }

    #[test]
    #[serial]
    fn it_should_store_multiple_settings() {
        let mut app1 = App::new();
        let u32_1: u32 = rand::random::<u32>();
        let u32_2: u32 = rand::random::<u32>();
        app1.add_plugins(SettingsPlugin::<TestSetting1>::new(
            "Bevy Settings Test Corp",
            "Some Game File 1",
        ));
        app1.add_plugins(SettingsPlugin::<TestSetting2>::new(
            "Bevy Settings Test Corp",
            "Some Game File 2",
        ));
        app1.add_systems(
            Update,
            move |mut writer: EventWriter<PersistSettings>,
                  mut test_setting_1: ResMut<TestSetting1>,
                  mut test_setting_2: ResMut<TestSetting2>| {
                *test_setting_1 = TestSetting1 { test: u32_1 };
                *test_setting_2 = TestSetting2 { test: u32_2 };
                writer.send(PersistSettings);
            },
        );
        app1.update(); // send event
        app1.update(); // react to persist

        let mut app2 = App::new();
        app2.add_plugins(SettingsPlugin::<TestSetting1>::new(
            "Bevy Settings Test Corp",
            "Some Game File 1",
        ));
        app2.add_plugins(SettingsPlugin::<TestSetting2>::new(
            "Bevy Settings Test Corp",
            "Some Game File 2",
        ));
        app2.update();
        let test_setting_1 = app2.world.resource::<TestSetting1>();
        assert_eq!(test_setting_1.test, u32_1);
        let test_setting_2 = app2.world.resource::<TestSetting2>();
        assert_eq!(test_setting_2.test, u32_2);
    }

    #[test]
    #[serial]
    fn it_should_store_singular_settings() {
        let mut app1 = App::new();
        let u32_1: u32 = rand::random::<u32>();
        let u32_2: u32 = rand::random::<u32>();
        app1.add_plugins(SettingsPlugin::<TestSetting1>::new(
            "Bevy Settings Test Corp",
            "Some Game File 1",
        ));
        app1.add_plugins(SettingsPlugin::<TestSetting2>::new(
            "Bevy Settings Test Corp",
            "Some Game File 2",
        ));
        app1.add_systems(
            Update,
            move |mut writer: EventWriter<PersistSetting<TestSetting1>>,
                  mut test_setting_1: ResMut<TestSetting1>,
                  mut test_setting_2: ResMut<TestSetting2>| {
                *test_setting_1 = TestSetting1 { test: u32_1 };
                *test_setting_2 = TestSetting2 { test: u32_2 };
                writer.send(PersistSetting::default());
            },
        );
        app1.update(); // send event
        app1.update(); // react to persist

        let mut app2 = App::new();
        app2.add_plugins(SettingsPlugin::<TestSetting1>::new(
            "Bevy Settings Test Corp",
            "Some Game File 1",
        ));
        app2.add_plugins(SettingsPlugin::<TestSetting2>::new(
            "Bevy Settings Test Corp",
            "Some Game File 2",
        ));
        app2.update();
        let test_setting_1 = app2.world.resource::<TestSetting1>();
        assert_eq!(test_setting_1.test, u32_1);
        let test_setting_2 = app2.world.resource::<TestSetting2>();
        assert_ne!(test_setting_2.test, u32_2);
    }

    #[test]
    #[serial]
    fn it_sould_trigger_autosave() {
        let mut app = App::new();
        app.add_plugins(SettingsPlugin::<TestSetting1>::new(
                "Bevy Settings Test Corp",
                "Some Game File 1",
                ).autosave());
        app.add_systems(PreUpdate, move |mut setting: ResMut<TestSetting1>| {
            setting.test = 9;
        });
        app.update();

        let mut app2 = App::new();
        app2.add_plugins(SettingsPlugin::<TestSetting1>::new(
                "Bevy Settings Test Corp",
                "Some Game File 1",
                ));
        app2.update();
        let test_setting = app2.world.resource::<TestSetting1>();
        assert_eq!(test_setting.test, 9);
    }
}
