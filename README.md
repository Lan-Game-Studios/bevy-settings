# Bevy Settings

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Doc](https://docs.rs/bevy-settings/badge.svg)](https://docs.rs/bevy-settings)
[![Crate](https://img.shields.io/crates/v/bevy-settings.svg)](https://crates.io/crates/bevy-settings)
[![Build Status](https://github.com/Lan-Game-Studios/bevy-settings/actions/workflows/rust.yml/badge.svg)](https://github.com/Lan-Game-Studios/bevy-settings/actions/workflows/rust.yml)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-v0.14-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![dependency status](https://deps.rs/repo/github/Lan-Game-Studios/bevy-settings/status.svg)](https://deps.rs/repo/github/Lan-Game-Studios/bevy-settings)

The goal of this project is to store settings in a resource throughout game launches.

[![Discord](https://assets-global.website-files.com/6257adef93867e50d84d30e2/636e0b5061df29d55a92d945_full_logo_blurple_RGB.svg)](https://discord.gg/JN5c3vrp) 

Currently this crate supports Linux, Mac and Windows.

The crate will choose the appropriate path for each OS to store the config file.

## TODO

- [x] multi storage support
- [ ] file naming support
- [ ] obfuscation support, this should just make it minimal hard to change the data, it is not really secure

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

## Known limitations

- the toml crate has problems with large numbers e.g. u64::MAX 
- there is a problem with tuple structs e.g. `TestSetting(u32)` does not work but `TestSetting{ test: u32 }` works fine.

Checkout the basic example to see how to persist the configuration.

| Version | Bevy Version |
|---------|--------------|
| 0.1.0   | 0.9          |
| 0.2.0   | 0.10         |
| 0.3.1   | 0.11         |
| 0.4.0   | 0.12         |
| 0.5.0   | 0.13         |
| 0.6.0   | 0.14         |

## Lan Game Studios

This crate is part of an effort to crate a game studio. Checkout 
[Mega Giga Cookie Destoryer TD](https://store.steampowered.com/app/2283070/Mega_Giga_Cookie_Destroyer_TD/) or
the mission of [Lan Game Studios](https://langamestudios.com) if you like games or game development.
