mod block;
mod canvas;

mod prelude {
    pub use crate::block::*;
    pub use crate::canvas::*;
    pub use bracket_lib::prelude::*;

    pub type Color = (u8, u8, u8);

    pub const SCREEN_WIDTH: i32 = 12;
    pub const SCREEN_HEIGHT: i32 = 25;
    pub const SCALE: i32 = 3;
    pub const ANIMATION_DURATION: usize = 16;
    pub const BLOCK_GLYPHS: [char; 16] = [
        '█', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░',
    ];
}

use prelude::*;

struct State {
    frame_index: usize,
    animation_index: usize,
    canvas: Canvas,
    active_block: Block,
}

impl State {
    fn new() -> Self {
        Self {
            frame_index: 0,
            animation_index: 0,
            canvas: Canvas::new(),
            active_block: Block::spawn(),
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
                Block::spawn()
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

        self.canvas.render(ctx, self.animation_index);
        self.active_block.render(ctx);

        self.frame_index += 1;
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_fps_cap(30.)
        .with_dimensions(SCREEN_WIDTH * SCALE, SCREEN_HEIGHT * SCALE)
        .with_title("Bracktrix")
        .build()?;
    main_loop(ctx, State::new())
}
