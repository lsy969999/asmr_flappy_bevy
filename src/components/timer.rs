use bevy::prelude::*;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct BirdAniTimer(pub Timer);
