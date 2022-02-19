mod block_spawning;
mod collision;
mod gravity;
mod line_detection;
mod pixel_renderer;
mod player_input;

use crate::prelude::*;

pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(pixel_renderer::pixel_render_system())
        .add_system(gravity::gravity_system(0))
        .add_system(block_spawning::block_spawning_system())
        .add_system(line_detection::line_detection_system())
        .build()
}
