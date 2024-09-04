use crate::{
    components::{
        game::{
            Bg, Bird, Ground, GroundCollider, GuideParent, PauseBtn, Pipe, PipeParent, PipePoint,
            ScoreParent,
        },
        on::OnGame,
        resize::Resizable,
        timer::BirdAniTimer,
    },
    events::picking::{BirdJumpEvent, PauseBtnClickEvent},
    resources::{assets::FlappyAssets, resize::ResizeScale},
    states::my_states::{Gaming, MyStates},
};
use avian2d::prelude::*;
use bevy::{math::vec3, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_tweening::{Animator, AnimatorState};
use rand::Rng;

pub fn on_enter_game(
    mut commands: Commands,
    resize_scale: Res<ResizeScale>,
    assets: Res<FlappyAssets>,
    mut next_state: ResMut<NextState<MyStates>>,
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
        On::<Pointer<Down>>::send_event::<BirdJumpEvent>(),
    );
    let ground = (
        Name::new("ground"),
        Ground,
        SpriteBundle {
            texture: assets.ground.clone(),
            transform: Transform::from_xyz(0., -100., 3.),
            ..default()
        },
    );

    let ground_collider = (
        Name::new("ground collider"),
        GroundCollider,
        RigidBody::Static,
        Collider::rectangle(168., 56.),
        TransformBundle::from_transform(Transform::from_xyz(0., -100., 1.)),
    );

    let sky_collider = (
        Name::new("sky collider"),
        RigidBody::Static,
        Collider::rectangle(168., 56.),
        TransformBundle::from_transform(Transform::from_xyz(0., 180., 1.)),
    );

    let bird = (
        Name::new("bird"),
        LockedAxes::ROTATION_LOCKED,
        ColliderDensity(0.0),
        Mass(5.0),
        RigidBody::Static,
        Collider::circle(17. / 2.),
        BirdAniTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Bird,
        SpriteBundle {
            texture: assets.gen_bird_atlas_texture.clone(),
            transform: Transform::from_xyz(-30., 30., 3.),
            ..default()
        },
        TextureAtlas {
            index: 0,
            layout: assets.gen_bird_atlas_layout.clone(),
        },
    );

    let get_ready = (
        Name::new("get ready"),
        SpriteBundle {
            texture: assets.label_get_ready.clone(),
            transform: Transform::from_xyz(0., 60., 0.),
            ..default()
        },
    );

    let guide = (
        Name::new("guide"),
        SpriteBundle {
            texture: assets.instructions.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    );

    let guide_parent = (
        Name::new("guide parent"),
        GuideParent,
        SpatialBundle::from_transform(Transform::from_xyz(0., 0., 1.)),
    );

    let score_parent = (
        Name::new("score parent"),
        ScoreParent,
        SpatialBundle::from_transform(Transform::from_xyz(0., 110., 10.)),
    );
    let large_num_0 = (
        Name::new("num"),
        SpriteBundle {
            texture: assets.get_large_num("0"),
            ..default()
        },
    );

    let pause_btn = (
        Name::new("pause btn"),
        PauseBtn { is_paused: false },
        SpriteBundle {
            texture: assets.button_pause.clone(),
            transform: Transform::from_xyz(-55., 110., 10.),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<PauseBtnClickEvent>(),
    );

    let ry = rand::thread_rng().gen_range(-20.0..60.0);
    let pipe_parent = (
        Name::new("pipe parent"),
        PipeParent,
        SpatialBundle::from_transform(Transform::from_xyz(85., ry, 2.)),
    );
    let ry = rand::thread_rng().gen_range(-20.0..60.0);
    let pipe_parent2 = (
        Name::new("pipe parent2"),
        PipeParent,
        SpatialBundle::from_transform(Transform::from_xyz(170., ry, 2.)),
    );

    let pipe_top = (
        Name::new("pipe top"),
        Pipe,
        Sensor,
        RigidBody::Static,
        Collider::rectangle(26., 160.),
        SpriteBundle {
            texture: assets.pipe_green_top.clone(),
            transform: Transform::from_xyz(0., 115., 0.),
            ..default()
        },
    );

    let pipe_point = (
        Name::new("pipe point"),
        PipePoint,
        Sensor,
        Collider::rectangle(5., 60.),
        RigidBody::Static,
        TransformBundle::from_transform(Transform::from_xyz(0., 0., 0.)),
    );

    let pipe_bottom = (
        Name::new("pipe bottom"),
        Pipe,
        Sensor,
        RigidBody::Static,
        Collider::rectangle(26., 160.),
        SpriteBundle {
            texture: assets.pipe_green_bottom.clone(),
            transform: Transform::from_xyz(0., -115., 0.),
            ..default()
        },
    );

    commands.spawn(bg).with_children(|parent| {
        parent.spawn(pause_btn);
        parent.spawn(score_parent).with_children(|parent| {
            parent.spawn(large_num_0);
        });
        parent.spawn(bird);
        parent.spawn(guide_parent).with_children(|parent| {
            parent.spawn(get_ready);
            parent.spawn(guide);
        });

        parent.spawn(pipe_parent).with_children(|parent| {
            parent.spawn(pipe_top.clone());
            parent.spawn(pipe_bottom.clone());
            parent.spawn(pipe_point.clone());
        });
        parent.spawn(pipe_parent2).with_children(|parent| {
            parent.spawn(pipe_top);
            parent.spawn(pipe_bottom);
            parent.spawn(pipe_point);
        });

        parent.spawn(ground);
        parent.spawn(ground_collider);
        parent.spawn(sky_collider);
    });

    next_state.set(MyStates::Game(Gaming::Guide));
}

pub fn pause_btn_click(
    mut commands: Commands,
    assets: Res<FlappyAssets>,
    mut er_pause_click: EventReader<PauseBtnClickEvent>,
    mut q_pause: Query<(Entity, &mut PauseBtn), With<PauseBtn>>,
    mut next_state: ResMut<NextState<MyStates>>,
    q_guide: Query<&GuideParent>,
    mut time: ResMut<Time<Physics>>,
    mut animator: Query<&mut Animator<Transform>, With<Bird>>,
) {
    for _ in er_pause_click.read() {
        // info!("pause btn clicked!");
        if let Ok((entity, mut pause)) = q_pause.get_single_mut() {
            if pause.is_paused {
                pause.is_paused = false;
                commands.entity(entity).insert(assets.button_pause.clone());
                if q_guide.get_single().is_ok() {
                    // return to guide
                    next_state.set(MyStates::Game(Gaming::Guide));
                } else {
                    // return to game
                    next_state.set(MyStates::Game(Gaming::Game));
                }

                //avian
                time.unpause();

                //tween
                let mut a = animator.get_single_mut().unwrap();
                a.state = AnimatorState::Playing;
            } else {
                pause.is_paused = true;
                commands.entity(entity).insert(assets.button_resume.clone());
                next_state.set(MyStates::Game(Gaming::Pause));

                //avian
                time.pause();

                //tween
                let mut a = animator.get_single_mut().unwrap();
                a.state = AnimatorState::Paused;
            }
        }
    }
}
