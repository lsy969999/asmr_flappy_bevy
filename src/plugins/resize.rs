use bevy::prelude::*;

use crate::{resources::resize::ResizeScale, systems::resize::resize};

pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ResizeScale { scale: 0. })
            .add_systems(Update, resize);
    }
}
