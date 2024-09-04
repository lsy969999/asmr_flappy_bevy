use bevy::{math::vec3, prelude::*};

use crate::{
    components::{game::Bg, on::OnGame, resize::Resizable},
    resources::{assets::FlappyAssets, resize::ResizeScale},
};

pub fn on_enter_game(
    mut commands: Commands,
    resize_scale: Res<ResizeScale>,
    assets: Res<FlappyAssets>,
) {
    info!("on enter game!");
    let bg = (
        Name::new("bg"),
        OnGame,
        Bg,
        Resizable,
        SpriteBundle {
            texture: assets.background_day.clone(),
            transform: Transform::from_scale(vec3(resize_scale.scale, resize_scale.scale, 1.)),
            ..default()
        },
    );
    commands.spawn(bg);
}
