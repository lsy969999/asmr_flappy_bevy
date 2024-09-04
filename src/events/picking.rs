use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
pub struct PlayBtnClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for PlayBtnClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}

#[derive(Event)]
pub struct PauseBtnClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for PauseBtnClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}

#[derive(Event)]
pub struct BirdJumpEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Down>>> for BirdJumpEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}
