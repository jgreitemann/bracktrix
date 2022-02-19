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
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(RandomNumberGenerator::new());
        resources.insert(Difficulty {
            gravity_tick_speed: 4,
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

        Self {
            world,
            resources,
            systems: build_schedule(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.resources.insert(ctx.key);

        self.systems.execute(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_fps_cap(30.)
        .with_dimensions(
            (SCREEN_WIDTH * SCALE) as i32,
            (SCREEN_HEIGHT * SCALE) as i32,
        )
        .with_title("Bracktrix")
        .build()?;
    main_loop(ctx, State::new())
}
