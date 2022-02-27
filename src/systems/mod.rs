mod block_spawning;
mod collision;
mod gravity;
mod line_deletion;
mod line_detection;
mod menu_renderer;
mod pixel_renderer;
mod player_input;
mod scoreboard_renderer;

#[cfg(feature = "gamepad")]
mod gamepad_input;

use crate::prelude::*;

pub fn build_base_schedule() -> Schedule {
    Schedule::builder()
        .add_system(pixel_renderer::pixel_render_system())
        .add_system(scoreboard_renderer::scoreboard_render_system())
        .build()
}

trait GamepadSupport {
    fn add_gamepad_support_system(&mut self) -> &mut Self;
}

#[cfg(not(feature = "gamepad"))]
impl GamepadSupport for Builder {
    fn add_gamepad_support_system(&mut self) -> &mut Self {
        self
    }
}

#[cfg(feature = "gamepad")]
impl GamepadSupport for Builder {
    fn add_gamepad_support_system(&mut self) -> &mut Self {
        self.add_system(gamepad_input::gamepad_input_system(None))
    }
}

pub fn build_play_schedule() -> Schedule {
    let mut builder = Schedule::builder();
    if cfg!(feature = "gamepad") {}
    builder
        .add_gamepad_support_system()
        .add_system(player_input::player_input_system())
        .add_system(gravity::gravity_system(0))
        .add_system(block_spawning::block_spawning_system())
        .add_system(line_detection::line_detection_system())
        .add_system(line_deletion::line_deletion_system())
        .build()
}

pub fn build_menu_schedule() -> Schedule {
    Schedule::builder()
        .add_system(menu_renderer::menu_render_system())
        .build()
}
