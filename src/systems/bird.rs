use avian2d::prelude::*;
use bevy::{math::vec2, prelude::*};

use crate::{
    components::{
        game::{Bird, GuideParent},
        timer::BirdAniTimer,
    },
    events::picking::BirdJumpEvent,
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
        info!("bird jump! depth: {}", event.1);
        if event.1 == 990. {
            info!("pause btn click! jump skip");
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
        }
    }
}
