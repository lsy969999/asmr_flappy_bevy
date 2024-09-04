use bevy::prelude::*;

#[derive(Component)]
pub struct Bg;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct TitleParent;

#[derive(Component)]
pub struct Bird;

#[derive(Component, Debug)]
pub struct PauseBtn {
    pub is_paused: bool,
}

#[derive(Component)]
pub struct GuideParent;

#[derive(Component)]
pub struct PipeParent;

#[derive(Component, Clone)]
pub struct Pipe;

#[derive(Component, Clone)]
pub struct PipePoint;

#[derive(Component)]
pub struct GroundCollider;

#[derive(Component)]
pub struct ScoreParent;
