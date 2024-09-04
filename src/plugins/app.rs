use bevy::prelude::*;

use crate::states::my_states::{Loading, MyStates};

use super::{assets::AssetsPlugin, mask::MaskPlugin, menu::MenuPlugin};

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(MyStates::Load(Loading::Loading));

        app.add_plugins(AssetsPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(MaskPlugin);

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
