use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    resources::assets::FlappyAssets,
    states::my_states::{Loading, MyStates},
    systems::assets::asset_gen,
};
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(MyStates::Load(Loading::Loading))
                .continue_to_state(MyStates::Load(Loading::Gen))
                .load_collection::<FlappyAssets>(),
        )
        .add_systems(OnEnter(MyStates::Load(Loading::Gen)), asset_gen);
    }
}
