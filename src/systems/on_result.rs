use bevy::{color::palettes::css, math::vec3, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_tweening::{
    lens::{SpriteColorLens, TransformPositionLens},
    Animator, Delay, EaseFunction, Tween, TweenCompleted,
};
use rand::Rng;
use std::time::Duration;

use crate::{
    components::{
        game::{Bg, PanelBest, PanelParent, PanelScore, PauseBtn},
        mask::MaskCenter,
        on::OnGame,
        timer::{PanelScoreCountingAniTimer, SparkleAniTimer},
    },
    constant::{
        TWEEN_CALLBACK_DEATH_WHITE, TWEEN_CALLBACK_GAME_TO_MENU, TWEEN_CALLBACK_MASK_CENTER_BACK,
        TWEEN_CALLBACK_PANEL_UP,
    },
    events::picking::ResultOkBtnClickEvent,
    resources::{assets::FlappyAssets, game::GameConfig},
    states::my_states::MyStates,
};
pub fn on_enter_result(
    mut commands: Commands,
    mut q_mc: Query<(Entity, &mut Transform), With<MaskCenter>>,
    q_pause_btn: Query<Entity, With<PauseBtn>>,
) {
    // info!("on enter result");
    if let Ok(entity) = q_pause_btn.get_single() {
        commands.entity(entity).despawn_recursive();
    }
    if let Ok((entity, mut tr)) = q_mc.get_single_mut() {
        tr.translation.z = 100.;
        let tween1 = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(200),
            SpriteColorLens {
                start: Color::srgba_u8(0, 0, 0, 0),
                end: css::WHITE.into(),
            },
        )
        .with_completed_event(TWEEN_CALLBACK_DEATH_WHITE);
        let tween2 = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(200),
            SpriteColorLens {
                start: css::WHITE.into(),
                end: Color::srgba_u8(0, 0, 0, 0),
            },
        )
        .with_completed_event(TWEEN_CALLBACK_MASK_CENTER_BACK);
        let seq = tween1.then(tween2);
        commands.entity(entity).insert(Animator::new(seq));
    }
}

pub fn tween_callback_death_white(
    mut commands: Commands,
    q_bg: Query<Entity, With<Bg>>,
    mut er_tween: EventReader<TweenCompleted>,
    assets: Res<FlappyAssets>,
    config: Res<GameConfig>,
) {
    for event in er_tween.read() {
        if event.user_data == TWEEN_CALLBACK_DEATH_WHITE {
            let now_score = config.score;
            let best_score = 2;

            let game_over_tween1 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(1),
                TransformPositionLens {
                    start: vec3(0., 52., -4.),
                    end: vec3(0., 52., 4.),
                },
            );
            let game_over_tween2 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(100),
                TransformPositionLens {
                    start: vec3(0., 52., 4.),
                    end: vec3(0., 55., 4.),
                },
            );
            let game_over_tween3 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(200),
                TransformPositionLens {
                    start: vec3(0., 55., 4.),
                    end: vec3(0., 50., 4.),
                },
            );
            let game_over_seq = Delay::new(Duration::from_millis(500))
                .then(game_over_tween1)
                .then(game_over_tween2)
                .then(game_over_tween3);
            let game_over = (
                Name::new("game over"),
                SpriteBundle {
                    texture: assets.label_game_over.clone(),
                    transform: Transform::from_xyz(0., 52., -4.),
                    ..default()
                },
                Animator::new(game_over_seq),
            );

            let panel_parent_tween1 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(500),
                TransformPositionLens {
                    start: vec3(0., -200., 4.),
                    end: vec3(0., 0., 4.),
                },
            )
            .with_completed_event(TWEEN_CALLBACK_PANEL_UP);
            let panel_parent_seq =
                Delay::new(Duration::from_millis(1000)).then(panel_parent_tween1);
            let panel_parent = (
                Name::new("panel parent"),
                PanelParent {
                    score: 0,
                    best: best_score,
                },
                SpatialBundle::from_transform(Transform::from_xyz(0., -200., 4.)),
                Animator::new(panel_parent_seq),
            );

            let panel = (
                Name::new("panel"),
                SpriteBundle {
                    texture: assets.panel_score.clone(),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                },
            );

            let ok_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(1),
                TransformPositionLens {
                    start: vec3(0., -50., -4.),
                    end: vec3(0., -50., 4.),
                },
            );
            let ok_seq = Delay::new(Duration::from_millis(2000)).then(ok_tween);
            let button_ok = (
                Name::new("ok"),
                Animator::new(ok_seq),
                SpriteBundle {
                    texture: assets.button_ok.clone(),
                    transform: Transform::from_xyz(0., -50., -4.),
                    ..default()
                },
                On::<Pointer<Down>>::target_component_mut::<Transform>(|_event, transform| {
                    transform.translation.y = -51.;
                }),
                On::<Pointer<Up>>::target_component_mut::<Transform>(|_event, transform| {
                    transform.translation.y = -50.;
                }),
                On::<Pointer<DragEnd>>::target_component_mut::<Transform>(|_event, transform| {
                    transform.translation.y = -50.;
                }),
                On::<Pointer<Click>>::send_event::<ResultOkBtnClickEvent>(),
            );

            let panel_score = (
                Name::new("panel_score"),
                PanelScore,
                SpatialBundle::from_transform(Transform::from_xyz(35., 7., 5.)),
            );
            let panel_best = (
                Name::new("panel_best"),
                PanelBest,
                SpatialBundle::from_transform(Transform::from_xyz(35., -14., 5.)),
            );

            let panel_score_num_0 = (
                Name::new("num"),
                SpriteBundle {
                    texture: assets.get_middle_num("0"),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                },
            );

            let mut offset = 0.;
            let padding = 8.;
            let num_vec = best_score
                .to_string()
                .chars()
                .map(|c| c.to_string())
                .map(|str| {
                    let cid = commands
                        .spawn((
                            Name::new("num"),
                            SpriteBundle {
                                texture: assets.get_middle_num(&str),
                                transform: Transform::from_xyz(offset, 0., 0.),
                                ..default()
                            },
                        ))
                        .id();
                    offset += padding;
                    cid
                })
                .collect::<Vec<_>>();

            // let panel_best_num_0 = (
            //     Name::new("num"),
            //     SpriteBundle {
            //         texture: assets.get_middle_num("0"),
            //         transform: Transform::from_xyz(0., 0., 0.),
            //         ..default()
            //     },
            // );

            let medal_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(1),
                TransformPositionLens {
                    start: vec3(-32., -4., -5.),
                    end: vec3(-32., -4., 5.),
                },
            );
            let medal_seq = Delay::new(Duration::from_millis(2000)).then(medal_tween);
            let medal = (
                Name::new("medal"),
                Animator::new(medal_seq),
                SpriteBundle {
                    texture: if now_score >= 40 {
                        assets.medal_platinum.clone()
                    } else if now_score >= 30 {
                        assets.medal_gold.clone()
                    } else if now_score >= 20 {
                        assets.medal_silver.clone()
                    } else if now_score >= 10 {
                        assets.medal_bronze.clone()
                    } else {
                        assets.medal_bronze.clone()
                    },
                    transform: Transform::from_xyz(-32., -4., -5.),
                    ..default()
                },
            );
            let sparkle_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(1),
                TransformPositionLens {
                    start: vec3(-32., -4., -6.),
                    end: vec3(-32., -4., 6.),
                },
            );
            let sparkle_seq = Delay::new(Duration::from_millis(2000)).then(sparkle_tween);
            let sparkle = (
                Name::new("sparkle"),
                Animator::new(sparkle_seq),
                SparkleAniTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                SpriteBundle {
                    texture: assets.gen_sparkle_atlas_texture.clone(),
                    transform: Transform::from_xyz(-32., -4., -6.),
                    ..default()
                },
                TextureAtlas {
                    index: 0,
                    layout: assets.gen_sparkle_atlas_layout.clone(),
                },
            );
            let new_tween = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(1),
                TransformPositionLens {
                    start: vec3(18., -4., -6.),
                    end: vec3(18., -4., 6.),
                },
            );
            let new_seq = Delay::new(Duration::from_millis(2000)).then(new_tween);
            let new = (
                Name::new("new"),
                Animator::new(new_seq),
                SpriteBundle {
                    texture: assets.label_new.clone(),
                    transform: Transform::from_xyz(18., -4., -6.),
                    ..default()
                },
            );

            if let Ok(entity) = q_bg.get_single() {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn(game_over);
                    parent.spawn(panel_parent).with_children(|parent| {
                        parent.spawn(panel);
                        parent.spawn(panel_score).with_children(|parent| {
                            parent.spawn(panel_score_num_0);
                        });
                        parent.spawn(panel_best).push_children(&num_vec).insert(
                            Transform::from_xyz(35. - 1. * (offset - padding) / 2., -14., 5.),
                        );

                        if now_score >= 10 {
                            parent.spawn(medal);
                            parent.spawn(sparkle);
                        }

                        if now_score > best_score {
                            parent.spawn(new);
                        }
                    });
                    parent.spawn(button_ok);
                });
            }
        }
    }
}

pub fn result_ok_btn_click(
    mut er_result_ok_click: EventReader<ResultOkBtnClickEvent>,
    mut commands: Commands,
    mut q_mc: Query<(Entity, &mut Transform), With<MaskCenter>>,
    //
) {
    for _ in er_result_ok_click.read() {
        if let Ok((entity, mut tr)) = q_mc.get_single_mut() {
            tr.translation.z = 100.;
            let tween1 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(200),
                SpriteColorLens {
                    start: Color::srgba_u8(0, 0, 0, 0),
                    end: css::BLACK.into(),
                },
            )
            .with_completed_event(TWEEN_CALLBACK_GAME_TO_MENU);
            let tween2 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(200),
                SpriteColorLens {
                    start: css::BLACK.into(),
                    end: Color::srgba_u8(0, 0, 0, 0),
                },
            )
            .with_completed_event(TWEEN_CALLBACK_MASK_CENTER_BACK);
            let seq = tween1.then(tween2);
            commands.entity(entity).insert(Animator::new(seq));
        }
    }
}
pub fn tween_callback_game_to_menu(
    mut er_tween: EventReader<TweenCompleted>,
    mut next_state: ResMut<NextState<MyStates>>,
) {
    for event in er_tween.read() {
        if event.user_data == TWEEN_CALLBACK_GAME_TO_MENU {
            next_state.set(MyStates::MainMenu);
        }
    }
}

pub fn on_exit_result(
    mut commands: Commands,
    q_on_result: Query<Entity, With<OnGame>>,
    mut config: ResMut<GameConfig>,
) {
    config.score = 0;
    for entity in &q_on_result {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn sparkle_ani(
    time: Res<Time>,
    atlases: Res<Assets<TextureAtlasLayout>>,
    mut q_ani: Query<(&mut Transform, &mut TextureAtlas, &mut SparkleAniTimer)>,
) {
    if let Ok((mut tr, mut at, mut timer)) = q_ani.get_single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let lay_id = at.layout.id();
            let layot = atlases.get(lay_id).unwrap();
            at.index = (at.index + 1) % layot.textures.len();

            if at.index == 0 {
                let rx = rand::thread_rng().gen_range((28.0)..=(36.0));
                let ry = rand::thread_rng().gen_range((0.0)..=(8.0));

                tr.translation.x = rx * -1.;
                tr.translation.y = ry * -1.;
            }
        }
    }
}

pub fn panel_score_counting_ani(
    mut commands: Commands,
    assets: Res<FlappyAssets>,
    time: Res<Time>,
    mut q_ani: Query<&mut PanelScoreCountingAniTimer>,
    mut q_panel_parent: Query<&mut PanelParent>,
    q_panel_score: Query<(Entity, &Children), With<PanelScore>>,
    q_panel_best: Query<(Entity, &Children), With<PanelBest>>,
    config: Res<GameConfig>,
) {
    if let Ok(mut timer) = q_ani.get_single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut pp = q_panel_parent.single_mut();
            // info!("test");
            if pp.score < config.score {
                pp.score += 1;
                let mut offset = 0.;
                let padding = 8.;
                let num_vec = pp
                    .score
                    .to_string()
                    .chars()
                    .map(|c| c.to_string())
                    .map(|str| {
                        let cid = commands
                            .spawn((
                                Name::new("num"),
                                SpriteBundle {
                                    texture: assets.get_middle_num(&str),
                                    transform: Transform::from_xyz(offset, 0., 0.),
                                    ..default()
                                },
                            ))
                            .id();
                        offset += padding;
                        cid
                    })
                    .collect::<Vec<_>>();
                let (entity, children) = q_panel_score.single();
                for &entity in children.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                commands
                    .entity(entity)
                    .push_children(&num_vec)
                    .insert(Transform::from_xyz(
                        35. + -1. * (offset - padding) / 2.,
                        7.,
                        10.,
                    ));

                if pp.best < pp.score {
                    let mut offset = 0.;
                    let padding = 8.;
                    let num_vec = pp
                        .score
                        .to_string()
                        .chars()
                        .map(|c| c.to_string())
                        .map(|str| {
                            let cid = commands
                                .spawn((
                                    Name::new("num"),
                                    SpriteBundle {
                                        texture: assets.get_middle_num(&str),
                                        transform: Transform::from_xyz(offset, 0., 0.),
                                        ..default()
                                    },
                                ))
                                .id();
                            offset += padding;
                            cid
                        })
                        .collect::<Vec<_>>();

                    let (entity, children) = q_panel_best.single();
                    for &entity in children.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    commands
                        .entity(entity)
                        .push_children(&num_vec)
                        .insert(Transform::from_xyz(
                            35. + -1. * (offset - padding) / 2.,
                            -14.,
                            10.,
                        ));
                }
            }
        }
    }
}

pub fn tween_callback_panel_up(
    mut commands: Commands,
    mut er_tween: EventReader<TweenCompleted>,
    q_panel: Query<(Entity, &PanelParent), With<PanelParent>>,
    config: Res<GameConfig>,
) {
    for event in er_tween.read() {
        if event.user_data == TWEEN_CALLBACK_PANEL_UP {
            // info!("panel up!");
            if let Ok((entity, _pp)) = q_panel.get_single() {
                let divide = if config.score < 1 { 1 } else { config.score };
                // info!("divide {}", divide);
                // info!("(1 / divide) as f32 :: {}", (1. / divide as f32));
                commands
                    .entity(entity)
                    .insert(PanelScoreCountingAniTimer(Timer::from_seconds(
                        0.5 / divide as f32,
                        TimerMode::Repeating,
                    )));
            }
        }
    }
}
