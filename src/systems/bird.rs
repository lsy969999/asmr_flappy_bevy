use std::{f32::consts::PI, time::Duration};

use avian2d::prelude::*;
use bevy::{math::vec2, prelude::*};
use bevy_tweening::{lens::TransformRotationLens, Animator, Delay, EaseFunction, Tween};

use crate::{
    components::{
        game::{Bird, GroundCollider, GuideParent, Pipe, PipePoint},
        timer::BirdAniTimer,
    },
    events::{game::ScoreUpEvent, picking::BirdJumpEvent},
    resources::resize::ResizeScale,
    states::my_states::{Gaming, MyStates},
};

pub fn bird_ani(
    time: Res<Time>,
    atlases: Res<Assets<TextureAtlasLayout>>,
    mut q_ani: Query<(&mut TextureAtlas, &mut BirdAniTimer), With<Bird>>,
) {
    if let Ok((mut at, mut timer)) = q_ani.get_single_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let lay_id = at.layout.id();
            let layot = atlases.get(lay_id).unwrap();
            at.index = (at.index + 1) % layot.textures.len();
        }
    }
}

pub fn bird_jump(
    mut commands: Commands,
    mut er_bird_jump: EventReader<BirdJumpEvent>,
    q_guide: Query<Entity, With<GuideParent>>,
    mut next_state: ResMut<NextState<MyStates>>,
    q_bird: Query<Entity, With<Bird>>,
    resize_scale: Res<ResizeScale>,
) {
    for event in er_bird_jump.read() {
        // info!("bird jump! depth: {}", event.1);
        if event.1 == 990. {
            // info!("pause btn click! jump skip");
            return;
        }

        // guide remove
        if let Ok(entity) = q_guide.get_single() {
            commands.entity(entity).despawn_recursive();
            next_state.set(MyStates::Game(Gaming::Game));
            if let Ok(entity) = q_bird.get_single() {
                commands.entity(entity).insert(RigidBody::Dynamic);
            }
        }

        // impule
        if let Ok(entity) = q_bird.get_single() {
            let mut impulse = ExternalImpulse::default();
            impulse.apply_impulse(vec2(0., 800. * resize_scale.scale));
            commands
                .entity(entity)
                .insert(impulse)
                .insert(LinearVelocity::default())
                .insert(AngularVelocity::default());

            //bird tween
            let tween1 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(200),
                TransformRotationLens {
                    start: Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
                    end: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 4.),
                },
            );
            let delay = Delay::new(Duration::from_millis(100));
            let tween2 = Tween::new(
                EaseFunction::QuadraticInOut,
                Duration::from_millis(300),
                TransformRotationLens {
                    start: Quat::from_euler(EulerRot::XYZ, 0., 0., PI / 4.),
                    end: Quat::from_euler(EulerRot::XYZ, 0., 0., -PI / 2.),
                },
            );
            let seq = tween1.then(delay).then(tween2);
            commands.entity(entity).insert(Animator::new(seq));
        }
    }
}

pub fn bird_collide_check(
    mut commands: Commands,
    q_collider: Query<(Entity, &CollidingEntities), With<Bird>>,
    q_pipe: Query<&Pipe>,
    q_ground: Query<&GroundCollider>,
    q_pipe_point: Query<(Entity, &PipePoint)>,
    mut ew_score_up: EventWriter<ScoreUpEvent>,
    mut next_state: ResMut<NextState<MyStates>>,
) {
    if let Ok((_entity, colliding_entities)) = q_collider.get_single() {
        for &entity in colliding_entities.iter() {
            if q_pipe.get(entity).is_ok() || q_ground.get(entity).is_ok() {
                next_state.set(MyStates::Game(Gaming::Result));
            }

            if let Ok((entity, _)) = q_pipe_point.get(entity) {
                commands.entity(entity).despawn_recursive();
                ew_score_up.send(ScoreUpEvent);
            }
        }
    }
}
