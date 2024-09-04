use bevy::prelude::*;

use crate::{components::game::Bg, resources::assets::FlappyAssets};

pub fn on_enter_menu(mut commands: Commands, assets: Res<FlappyAssets>) {
    info!("on enter menu!!");

    let bg = (
        Name::new("bg"),
        Bg,
        SpriteBundle {
            texture: assets.background_day.clone(),
            ..default()
        },
    );

    commands.spawn(bg);
}
