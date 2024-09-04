use bevy::prelude::*;

use crate::{
    events::picking::PlayBtnClickEvent,
    states::my_states::MyStates,
    systems::{
        bird::bird_ani,
        ground::ground_movement,
        on_menu::{
            on_enter_menu, on_exit_menu, play_btn_click, title_movement,
            tween_callback_menu_to_game,
        },
    },
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayBtnClickEvent>()
            .add_systems(OnEnter(MyStates::MainMenu), on_enter_menu)
            .add_systems(OnExit(MyStates::MainMenu), on_exit_menu)
            .add_systems(
                Update,
                (
                    title_movement,
                    ground_movement,
                    bird_ani,
                    play_btn_click,
                    tween_callback_menu_to_game,
                )
                    .run_if(in_state(MyStates::MainMenu)),
            );
    }
}
