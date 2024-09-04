use bevy::prelude::*;

use crate::{
    events::{
        game::ScoreUpEvent,
        picking::{BirdJumpEvent, PauseBtnClickEvent},
    },
    resources::game::GameConfig,
    states::my_states::{Gaming, MyStates},
    systems::{
        bird::{bird_ani, bird_collide_check, bird_jump},
        ground::ground_movement,
        on_game::{on_enter_game, pause_btn_click},
        pipe::pipe_movement,
        score::score_up,
    },
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PauseBtnClickEvent>()
            .add_event::<BirdJumpEvent>()
            .add_event::<ScoreUpEvent>()
            .insert_resource(GameConfig { score: 0 })
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
            )
            .add_systems(
                Update,
                (pipe_movement, bird_collide_check, score_up)
                    .run_if(in_state(MyStates::Game(Gaming::Game))),
            );
    }
}
