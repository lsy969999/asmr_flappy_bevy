use asmr_flappy_bevy::AppPlugin;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppPlugin)
        .run();
}
