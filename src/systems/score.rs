use bevy::prelude::*;

use crate::{
    components::game::ScoreParent,
    events::game::ScoreUpEvent,
    resources::{assets::FlappyAssets, game::GameConfig},
};

pub fn score_up(
    mut commands: Commands,
    mut er_score_up: EventReader<ScoreUpEvent>,
    mut config: ResMut<GameConfig>,
    assets: Res<FlappyAssets>,
    q_score_parent: Query<(Entity, &Children), With<ScoreParent>>,
) {
    for _ in er_score_up.read() {
        config.score += 1;

        if let Ok((entity, children)) = q_score_parent.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }
            let mut offset = 0.;
            let padding = 13.;
            let num_vec = config
                .score
                .to_string()
                .chars()
                .map(|c| c.to_string())
                .map(|str| {
                    let cid = commands
                        .spawn((
                            Name::new("num"),
                            SpriteBundle {
                                texture: assets.get_large_num(&str),
                                transform: Transform::from_xyz(offset, 0., 0.),
                                ..default()
                            },
                        ))
                        .id();
                    offset += padding;
                    cid
                })
                .collect::<Vec<_>>();

            commands
                .entity(entity)
                .push_children(&num_vec)
                .insert(Transform::from_xyz(
                    -1. * (offset - padding) / 2.,
                    110.,
                    10.,
                ));
        }
    }
}
