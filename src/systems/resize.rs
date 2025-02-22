use crate::{
    components::resize::Resizable,
    constant::{ORIGINAL_HEIGHT, ORIGINAL_WIDTH},
    resources::resize::ResizeScale,
};
use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResized};

pub fn resize(
    mut commands: Commands,
    mut resize_reader: EventReader<WindowResized>,
    mut resize_scale: ResMut<ResizeScale>,
    mut q_resizable: Query<&mut Transform, With<Resizable>>,
) {
    for event in resize_reader.read() {
        let scale_x = event.width / ORIGINAL_WIDTH;
        let scale_y = event.height / ORIGINAL_HEIGHT;
        let scale = scale_x.min(scale_y);
        resize_scale.scale = scale;
        // info!("resize!! scale: {}", scale);
        for mut tr in &mut q_resizable {
            tr.scale.x = scale;
            tr.scale.y = scale;
        }
        commands.insert_resource(Gravity(Vec2::NEG_Y * 800.0 * scale))
    }
}
