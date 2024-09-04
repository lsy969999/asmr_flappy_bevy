use bevy::prelude::*;

use crate::components::{game::Bird, timer::BirdAniTimer};

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
