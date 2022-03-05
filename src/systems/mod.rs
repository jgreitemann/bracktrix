mod block_spawning;
mod collision;
mod game_input;
mod gravity;
mod line_deletion;
mod line_detection;
mod menu_action;
mod menu_input;
mod menu_renderer;
mod pixel_renderer;
mod scoreboard_renderer;

use crate::prelude::*;

pub fn build_base_schedule() -> Schedule {
    Schedule::builder()
        .add_system(pixel_renderer::pixel_render_system())
        .build()
}

pub fn build_play_schedule() -> Schedule {
    Schedule::builder()
        .add_system(game_input::game_input_system(GameInputState::new()))
        .add_system(gravity::gravity_system(0))
        .add_system(block_spawning::block_spawning_system())
        .add_system(line_detection::line_detection_system())
        .add_system(line_deletion::line_deletion_system())
        .add_system(scoreboard_renderer::scoreboard_render_system())
        .build()
}

pub fn build_menu_schedule() -> Schedule {
    Schedule::builder()
        .add_system(menu_input::menu_input_system(MenuInputState::new()))
        .flush()
        .add_system(menu_renderer::menu_render_system())
        .add_system(menu_action::menu_action_system())
        .build()
}
