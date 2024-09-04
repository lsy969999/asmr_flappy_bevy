use bevy::{color::palettes::css, prelude::*};
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::SpriteColorLens, Animator, EaseFunction, Tween, TweenCompleted};
use std::time::Duration;

use crate::{
    components::{
        game::{Bg, PauseBtn},
        mask::MaskCenter,
        on::OnGame,
    },
    constant::{
        TWEEN_CALLBACK_DEATH_WHITE, TWEEN_CALLBACK_GAME_TO_MENU, TWEEN_CALLBACK_MASK_CENTER_BACK,
    },
    events::picking::ResultOkBtnClickEvent,
    resources::assets::FlappyAssets,
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
) {
    for event in er_tween.read() {
        if event.user_data == TWEEN_CALLBACK_DEATH_WHITE {
            //
            let game_over = (
                Name::new("game over"),
                SpriteBundle {
                    texture: assets.label_game_over.clone(),
                    transform: Transform::from_xyz(0., 50., 4.),
                    ..default()
                },
            );

            let panel_parent = (
                Name::new("panel parent"),
                SpatialBundle::from_transform(Transform::from_xyz(0., 0., 4.)),
            );

            let panel = (
                Name::new("panel"),
                SpriteBundle {
                    texture: assets.panel_score.clone(),
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..default()
                },
            );

            let button_ok = (
                Name::new("ok"),
                SpriteBundle {
                    texture: assets.button_ok.clone(),
                    transform: Transform::from_xyz(0., -50., 4.),
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

            if let Ok(entity) = q_bg.get_single() {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn(game_over);
                    parent.spawn(panel_parent).with_children(|parent| {
                        parent.spawn(panel);
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

pub fn on_exit_result(mut commands: Commands, q_on_result: Query<Entity, With<OnGame>>) {
    for entity in &q_on_result {
        commands.entity(entity).despawn_recursive();
    }
}
