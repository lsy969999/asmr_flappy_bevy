use bevy::prelude::*;

use crate::{
    events::picking::{BirdJumpEvent, PauseBtnClickEvent},
    states::my_states::{Gaming, MyStates},
    systems::{
        bird::{bird_ani, bird_jump},
        ground::ground_movement,
        on_game::{on_enter_game, pause_btn_click},
    },
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PauseBtnClickEvent>()
            .add_event::<BirdJumpEvent>()
            .add_systems(OnEnter(MyStates::Game(Gaming::Init)), on_enter_game)
            .add_systems(
                Update,
                (ground_movement, bird_ani, bird_jump).run_if(
                    in_state(MyStates::Game(Gaming::Guide))
                        .or_else(in_state(MyStates::Game(Gaming::Game))),
                ),
            )
            .add_systems(
                Update,
                pause_btn_click.run_if(
                    in_state(MyStates::Game(Gaming::Guide)).or_else(
                        in_state(MyStates::Game(Gaming::Game))
                            .or_else(in_state(MyStates::Game(Gaming::Pause))),
                    ),
                ),
            );
    }
}
