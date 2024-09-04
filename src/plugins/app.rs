use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_tweening::TweeningPlugin;

use crate::states::my_states::{Loading, MyStates};

use super::{
    assets::AssetsPlugin, game::GamePlugin, mask::MaskPlugin, menu::MenuPlugin,
    resize::ResizePlugin,
};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MyStates::Load(Loading::Loading));

        app.add_plugins(DefaultPickingPlugins)
            .add_plugins(TweeningPlugin);

        app.add_plugins(AssetsPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(MaskPlugin)
            .add_plugins(ResizePlugin)
            .add_plugins(GamePlugin);

        #[cfg(feature = "inspector")]
        {
            use super::inspector::InspectorPlugin;
            app.add_plugins(InspectorPlugin);
        }

        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        });
    }
}
