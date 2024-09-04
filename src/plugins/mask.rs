use bevy::prelude::*;

use crate::systems::mask::mask_setup;

pub struct MaskPlugin;

impl Plugin for MaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mask_setup);
    }
}
