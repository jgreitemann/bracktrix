mod block;
mod components;
mod graphics;
mod resources;
mod scaffold;
mod systems;

#[cfg(test)]
mod test_utils;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::block::*;
    pub use crate::components::*;
    pub use crate::graphics::*;
    pub use crate::resources::*;
    pub use crate::scaffold::*;
    pub use crate::systems::*;

    pub const SCREEN_WIDTH: usize = 23;
    pub const SCREEN_HEIGHT: usize = 25;
    pub const CANVAS_WIDTH: usize = 12;
    pub const CANVAS_HEIGHT: usize = 21;
}

use prelude::*;

const SCALE: usize = 3;

struct State {
    world: World,
    resources: Resources,
    base_systems: Schedule,
    play_systems: Schedule,
    menu_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(GameMode::Play);
        resources.insert(RandomNumberGenerator::new());
        resources.insert(Difficulty {
            gravity_tick_speed: 8,
        });

        let scaffold = Scaffold {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            canvas_width: CANVAS_WIDTH,
            canvas_height: CANVAS_HEIGHT,
        };
        resources.insert(Screen(scaffold.screen_rect()));
        resources.insert(BlockSpawnPoints {
            active_block_spawn: scaffold.spawn_point(),
            preview_block_spawn: scaffold.preview_origin(),
        });

        world.extend(scaffold.border_entities());

        world.push((DisplayText("Game Over!".to_string()),));

        Self {
            world,
            resources,
            base_systems: build_base_schedule(),
            play_systems: build_play_schedule(),
            menu_systems: build_menu_schedule(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls_bg((0, 0, 0, 0));

        self.resources.insert(ctx.key);

        let mode = *self.resources.get::<GameMode>().unwrap();
        self.base_systems
            .execute(&mut self.world, &mut self.resources);
        match mode {
            GameMode::Play => &mut self.play_systems,
            GameMode::Menu => &mut self.menu_systems,
        }
        .execute(&mut self.world, &mut self.resources);

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_fps_cap(60.)
        .with_title("Bracktrix")
        .with_dimensions(
            (SCREEN_WIDTH * SCALE) as i32,
            (SCREEN_HEIGHT * SCALE) as i32,
        )
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png")
        .with_simple_console(2 * SCREEN_WIDTH, 2 * SCREEN_HEIGHT, "terminal8x8.png")
        .build()?;
    main_loop(ctx, State::new())
}
