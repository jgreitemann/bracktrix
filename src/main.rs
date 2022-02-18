mod block;
mod canvas;
mod components;
mod graphics;
mod resources;
mod scaffold;
mod systems;
mod viewport;

#[cfg(test)]
mod test_utils;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::block::*;
    pub use crate::canvas::*;
    pub use crate::components::*;
    pub use crate::graphics::*;
    pub use crate::resources::*;
    pub use crate::scaffold::*;
    pub use crate::systems::*;
    pub use crate::viewport::*;

    pub const ANIMATION_DURATION: usize = 16;
    pub const BLOCK_GLYPHS: [char; 16] = [
        '█', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░',
    ];
}

use prelude::*;

const SCREEN_WIDTH: usize = 23;
const SCREEN_HEIGHT: usize = 25;
const CANVAS_WIDTH: usize = 12;
const CANVAS_HEIGHT: usize = 21;
const SCALE: usize = 3;

struct State {
    world: World,
    resources: Resources,
    systems: Schedule,
    frame_index: usize,
    animation_index: usize,
    canvas: Canvas,
}

impl State {
    fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();

        let scaffold = Scaffold {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            canvas_width: CANVAS_WIDTH,
            canvas_height: CANVAS_HEIGHT,
        };
        resources.insert(Screen(scaffold.screen_rect()));
        world.extend(scaffold.border_entities());

        let canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

        let block = BlockShape::random();
        world.extend(block.pixels().into_iter().map(|pix| {
            (
                Active {},
                Position(pix.position + scaffold.spawn_point()),
                Pivot(pix.position * 2 - block.rotation_offset()),
                PixelRender {
                    colors: ColorPair::new(pix.color, BLACK),
                    glyph: to_cp437(pix.glyph),
                },
            )
        }));

        let preview_block = BlockShape::random();
        world.extend(preview_block.pixels().into_iter().map(|pix| {
            (
                Preview {},
                Position(pix.position + scaffold.preview_origin()),
                Pivot(pix.position * 2 - preview_block.rotation_offset()),
                PixelRender {
                    colors: ColorPair::new(pix.color, BLACK),
                    glyph: to_cp437(pix.glyph),
                },
            )
        }));

        Self {
            world,
            resources,
            systems: build_schedule(),
            frame_index: 0,
            animation_index: 0,
            canvas,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        /*
        if self.animation_index == 0 {
            let mut updated = self.active_block.clone().with_keys_applied(ctx);

            if !self.block_fits_canvas(&updated) {
                // roll back
                updated = self.active_block.clone();
            }

            updated = updated.with_gravity_applied(self.frame_index);
            self.active_block = if self.block_fits_canvas(&updated) {
                updated
            } else {
                self.canvas.bake(self.active_block.pixels());
                let next = Block::new(self.preview_block.shape(), self.canvas.spawn_point());
                self.preview_block =
                    Block::new(BlockShape::random(), self.scaffold.preview_origin());
                next
            };
        }

        let full_rows = self.canvas.full_rows();

        if self.animation_index == 0 && !full_rows.is_empty() {
            self.animation_index = ANIMATION_DURATION;
        }

        if self.animation_index > 0 {
            self.animation_index -= 1;
            if self.animation_index == 0 {
                self.canvas.clear_rows(full_rows.into_iter());
            }
        }

         */

        self.resources.insert(ctx.key);
        self.resources.insert(Frame {
            index: self.frame_index,
        });

        // self.canvas
        //     .render(self.scaffold.canvas_viewport(ctx), self.animation_index);
        self.systems.execute(&mut self.world, &mut self.resources);
        render_draw_buffer(ctx).expect("Render error");

        self.frame_index += 1;
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
