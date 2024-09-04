use bevy::{color::palettes::css, math::vec2, prelude::*};

use crate::{
    components::{mask::MaskCenter, resize::Resizable},
    constant::{ORIGINAL_HEIGHT, ORIGINAL_WIDTH},
};

pub fn mask_setup(mut commands: Commands) {
    let mask_small = 200.;
    let mask_large = 600.;
    let mask_tr_z = -1.;
    let mask_tr_x = (ORIGINAL_WIDTH / 2.) + (mask_small / 2.);
    let mask_tr_y = (ORIGINAL_HEIGHT / 2.) + (mask_small / 2.);
    let mask_center_z = -1.;
    let mask_parent = (
        Name::new("mask parent"),
        Resizable,
        SpatialBundle::from_transform(Transform::from_xyz(0., 0., 0.)),
    );
    let left = (
        Name::new("mask left"),
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(vec2(mask_small, mask_large)),
                ..default()
            },
            transform: Transform::from_xyz(-mask_tr_x, 0., mask_tr_z),
            ..default()
        },
    );
    let right = (
        Name::new("mask right"),
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(vec2(mask_small, mask_large)),
                ..default()
            },
            transform: Transform::from_xyz(mask_tr_x, 0., mask_tr_z),
            ..default()
        },
    );
    let up: (Name, SpriteBundle) = (
        Name::new("mask up"),
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(vec2(mask_large, mask_small)),
                ..default()
            },
            transform: Transform::from_xyz(0., mask_tr_y, mask_tr_z),
            ..default()
        },
    );
    let down: (Name, SpriteBundle) = (
        Name::new("mask down"),
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(vec2(mask_large, mask_small)),
                ..default()
            },
            transform: Transform::from_xyz(0., -mask_tr_y, mask_tr_z),
            ..default()
        },
    );

    let mask_center = (
        Name::new("mask center"),
        Resizable,
        MaskCenter,
        SpriteBundle {
            sprite: Sprite {
                color: css::DIM_GRAY.into(),
                custom_size: Some(vec2(ORIGINAL_WIDTH, ORIGINAL_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0., 0., mask_center_z),
            ..default()
        },
    );
    commands.spawn(mask_parent).with_children(|parent| {
        parent.spawn(left);
        parent.spawn(right);
        parent.spawn(up);
        parent.spawn(down);
    });
    commands.spawn(mask_center);
}
