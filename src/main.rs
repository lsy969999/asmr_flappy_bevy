use asmr_flappy_bevy::{AppPlugin, ORIGINAL_HEIGHT, ORIGINAL_WIDTH};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (ORIGINAL_WIDTH * 3., ORIGINAL_HEIGHT * 3.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(AppPlugin)
        .run();
}
