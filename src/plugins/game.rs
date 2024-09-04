use bevy::prelude::*;

use crate::{
    events::{
        game::ScoreUpEvent,
        picking::{BirdJumpEvent, PauseBtnClickEvent, ResultOkBtnClickEvent},
    },
    resources::game::GameConfig,
    states::my_states::{Gaming, MyStates},
    systems::{
        bird::{bird_ani, bird_collide_check, bird_jump},
        ground::ground_movement,
        on_game::{on_enter_game, pause_btn_click},
        on_result::{
            on_enter_result, on_exit_result, panel_score_counting_ani, result_ok_btn_click,
            sparkle_ani, tween_callback_death_white, tween_callback_game_to_menu,
            tween_callback_panel_up,
        },
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
            .add_event::<ResultOkBtnClickEvent>()
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
            )
            .add_systems(OnEnter(MyStates::Game(Gaming::Result)), on_enter_result)
            .add_systems(OnExit(MyStates::Game(Gaming::Result)), on_exit_result)
            .add_systems(
                Update,
                (
                    tween_callback_death_white,
                    result_ok_btn_click,
                    tween_callback_game_to_menu,
                    sparkle_ani,
                    panel_score_counting_ani,
                    tween_callback_panel_up,
                )
                    .run_if(in_state(MyStates::Game(Gaming::Result))),
            );
    }
}
