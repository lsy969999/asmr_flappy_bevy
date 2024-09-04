use bevy::prelude::*;

use crate::components::game::Ground;

pub fn ground_movement(time: Res<Time>, mut q_ground: Query<&mut Transform, With<Ground>>) {
    if let Ok(mut tr) = q_ground.get_single_mut() {
        tr.translation.x -= time.delta_seconds() * 50.;
        if tr.translation.x <= -12. {
            tr.translation.x = 0.;
        }
    }
}
