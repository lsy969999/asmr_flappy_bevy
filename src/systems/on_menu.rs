use bevy::{math::vec3, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{
    components::{
        game::{Bg, Bird, Ground, TitleParent},
        on::OnMenu,
        resize::Resizable,
        timer::BirdAniTimer,
    },
    events::picking::PlayBtnClickEvent,
    resources::{assets::FlappyAssets, resize::ResizeScale},
    states::my_states::{Gaming, MyStates},
};

pub fn on_enter_menu(
    mut commands: Commands,
    assets: Res<FlappyAssets>,
    resize_scale: Res<ResizeScale>,
) {
    // info!("on enter menu!!");
    let bg = (
        Name::new("bg"),
        OnMenu,
        Bg,
        Resizable,
        SpriteBundle {
            texture: assets.background_day.clone(),
            transform: Transform::from_scale(vec3(resize_scale.scale, resize_scale.scale, 1.)),
            ..default()
        },
    );

    let ground = (
        Name::new("ground"),
        Ground,
        SpriteBundle {
            texture: assets.ground.clone(),
            transform: Transform::from_xyz(0., -100., 1.),
            ..default()
        },
    );

    let title_parent = (
        Name::new("title parent"),
        TitleParent,
        SpatialBundle::from_transform(Transform::from_xyz(0., 60., 1.)),
    );

    let title = (
        Name::new("title"),
        SpriteBundle {
            texture: assets.label_flappy_bird.clone(),
            transform: Transform::from_xyz(-10., 0., 0.),
            ..default()
        },
    );

    let bird = (
        Name::new("bird"),
        BirdAniTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Bird,
        SpriteBundle {
            texture: assets.gen_bird_atlas_texture.clone(),
            transform: Transform::from_xyz(50., 0., 0.),
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: assets.gen_bird_atlas_layout.clone(),
        },
    );

    let play_presed = assets.button_play_pressed.clone();
    let play_normal = assets.button_play_normal.clone();
    let play_normal2 = assets.button_play_normal.clone();
    let paly_btn = (
        Name::new("play btn"),
        SpriteBundle {
            texture: assets.button_play_normal.clone(),
            transform: Transform::from_xyz(0., -30., 1.),
            ..default()
        },
        On::<Pointer<Down>>::target_commands_mut(move |_event, commands| {
            commands.insert(play_presed.clone());
        }),
        On::<Pointer<Up>>::target_commands_mut(move |_event, commands| {
            commands.insert(play_normal.clone());
        }),
        On::<Pointer<DragEnd>>::target_commands_mut(move |_event, commands| {
            commands.insert(play_normal2.clone());
        }),
        On::<Pointer<Click>>::send_event::<PlayBtnClickEvent>(),
    );

    commands.spawn(bg).with_children(|parent| {
        parent.spawn(title_parent).with_children(|parent| {
            parent.spawn(title);
            parent.spawn(bird);
        });
        parent.spawn(paly_btn);
        parent.spawn(ground);
    });
}

pub fn title_movement(mut q_title: Query<&mut Transform, With<TitleParent>>, time: Res<Time>) {
    if let Ok(mut tr) = q_title.get_single_mut() {
        tr.translation.y = 60. + (time.elapsed_seconds() * 5.).sin() * 2.;
    }
}

pub fn play_btn_click(
    mut er_play_click: EventReader<PlayBtnClickEvent>,
    mut next_state: ResMut<NextState<MyStates>>,
) {
    for _ in er_play_click.read() {
        // info!("paly btn click!!");
        next_state.set(MyStates::Game(Gaming::Init));
    }
}

pub fn on_exit_menu(mut commands: Commands, q_menu: Query<Entity, With<OnMenu>>) {
    for entity in &q_menu {
        commands.entity(entity).despawn_recursive();
    }
}
