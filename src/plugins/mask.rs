use bevy::prelude::*;
use bevy_tweening::TweenCompleted;

use crate::systems::mask::{mask_setup, tween_callback_mask_center_back};

pub struct MaskPlugin;

impl Plugin for MaskPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mask_setup).add_systems(
            Update,
            tween_callback_mask_center_back.run_if(on_event::<TweenCompleted>()),
        );
    }
}
