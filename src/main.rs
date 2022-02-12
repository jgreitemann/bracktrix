mod block;
mod canvas;
mod graphics;
mod scaffold;
mod viewport;

#[cfg(test)]
mod test_utils;

mod prelude {
    pub use crate::block::*;
    pub use crate::canvas::*;
    pub use crate::graphics::*;
    pub use crate::scaffold::*;
    pub use crate::viewport::*;
    pub use bracket_lib::prelude::*;

    pub const ANIMATION_DURATION: usize = 16;
    pub const BLOCK_GLYPHS: [char; 16] = [
        '█', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░',
    ];
}

use prelude::*;

const SCREEN_WIDTH: usize = 22;
const SCREEN_HEIGHT: usize = 25;
const CANVAS_WIDTH: usize = 12;
const CANVAS_HEIGHT: usize = 21;
const SCALE: usize = 3;

struct State {
    frame_index: usize,
    animation_index: usize,
    scaffold: Scaffold,
    canvas: Canvas,
    active_block: Block,
}

impl State {
    fn new() -> Self {
        let scaffold = Scaffold {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            canvas_width: CANVAS_WIDTH,
            canvas_height: CANVAS_HEIGHT,
        };

        let canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

        let active_block = Block::new(canvas.spawn_point());

        Self {
            frame_index: 0,
            animation_index: 0,
            scaffold,
            canvas,
            active_block,
        }
    }

    fn block_fits_canvas(&self, block: &Block) -> bool {
        block.points().all(|p| self.canvas.is_empty(&p))
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

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
                Block::new(self.canvas.spawn_point())
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

        self.scaffold.render(ctx);
        self.canvas
            .render(self.scaffold.canvas_viewport(ctx), self.animation_index);
        self.active_block.render(self.scaffold.canvas_viewport(ctx));

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
