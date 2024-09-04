use bevy::prelude::*;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct BirdAniTimer(pub Timer);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct SparkleAniTimer(pub Timer);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct PanelScoreCountingAniTimer(pub Timer);
