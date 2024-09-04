use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

#[derive(Event)]
#[allow(unused)]
pub struct PlayBtnClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for PlayBtnClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}

#[derive(Event)]
#[allow(unused)]
pub struct PauseBtnClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for PauseBtnClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}

#[derive(Event)]
#[allow(unused)]
pub struct BirdJumpEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Down>>> for BirdJumpEvent {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}

#[derive(Event)]
#[allow(unused)]
pub struct ResultOkBtnClickEvent(pub Entity, pub f32);

impl From<ListenerInput<Pointer<Click>>> for ResultOkBtnClickEvent {
    fn from(event: ListenerInput<Pointer<Click>>) -> Self {
        Self(event.target, event.hit.depth)
    }
}
