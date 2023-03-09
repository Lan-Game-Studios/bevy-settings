# Bevy Settings

The goal of this project is to store settings in a resource throughout game launches.

Currently this crate supports Linux, Mac and Windows.

The crate will choose the appropriate path for each OS to store the config file.

## Usage

> This example will generate a config file on your system but it probably will 
> not hurt you if you pick something non existent

```rust
use bevy::prelude::*; 
use bevy_settings::{Serialize, Deserialize};

#[derive(Resource, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "bevy_settings::serde")]
struct Settings {
    master_volume: f64,
    custom_cursor: bool,
}

fn main () {
    App::new()
        .add_plugin(bevy_settings::SettingsPlugin::<Settings>::new(
            "My awesome game studio",
            "The name of the game"
        ))
        .run();
}
```

on e.g. my linux machine this will create 

```
❯ cat ~/.config/myawesomegamestudio/My awesome game studio.toml 
master_volume = 0.0
custom_cursor = false
```

Checkout the basic example to see how to persist the configuration.

| Version | Bevy Version |
|---------|--------------|
| 0.1.0   | 0.9          |
| 0.2.0   | 0.10         |
