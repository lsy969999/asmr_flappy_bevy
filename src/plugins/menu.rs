use bevy::prelude::*;

use crate::{states::my_states::MyStates, systems::on_menu::on_enter_menu};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::MainMenu), on_enter_menu);
    }
}
