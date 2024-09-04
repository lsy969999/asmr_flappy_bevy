use avian2d::prelude::{Collider, RigidBody, Sensor};
use bevy::prelude::*;

use crate::components::game::{PipeParent, PipePoint};

pub fn pipe_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut q_pipe: Query<(Entity, &mut Transform), With<PipeParent>>,
) {
    for (entity, mut tr) in &mut q_pipe {
        tr.translation.x -= time.delta_seconds() * 50.;
        if tr.translation.x <= -85. {
            tr.translation.x = 85.;
            let pipe_point = (
                Name::new("pipe point"),
                PipePoint,
                Sensor,
                Collider::rectangle(5., 60.),
                RigidBody::Static,
                TransformBundle::from_transform(Transform::from_xyz(0., 0., 0.)),
            );
            commands.entity(entity).with_children(|parent| {
                parent.spawn(pipe_point);
            });
        }
    }
}
