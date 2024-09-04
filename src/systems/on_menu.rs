use bevy::{math::vec3, prelude::*};

use crate::{
    components::{game::Bg, resize::Resizable},
    resources::{assets::FlappyAssets, resize::ResizeScale},
};

pub fn on_enter_menu(
    mut commands: Commands,
    assets: Res<FlappyAssets>,
    resize_scale: Res<ResizeScale>,
) {
    info!("on enter menu!!");

    let bg = (
        Name::new("bg"),
        Bg,
        Resizable,
        SpriteBundle {
            texture: assets.background_day.clone(),
            transform: Transform::from_scale(vec3(resize_scale.scale, resize_scale.scale, 0.)),
            ..default()
        },
    );

    commands.spawn(bg);
}
