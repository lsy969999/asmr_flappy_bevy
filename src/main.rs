use asmr_flappy_bevy::AppPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(AppPlugin)
        .run();
}
