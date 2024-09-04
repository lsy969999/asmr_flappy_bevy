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
