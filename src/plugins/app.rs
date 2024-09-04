use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
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
