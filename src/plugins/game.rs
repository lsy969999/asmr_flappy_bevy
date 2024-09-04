use bevy::prelude::*;

use crate::{
    states::my_states::{Gaming, MyStates},
    systems::on_game::on_enter_game,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::Game(Gaming::Init)), on_enter_game);
    }
}
