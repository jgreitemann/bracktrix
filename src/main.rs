mod block;
mod canvas;

mod prelude {
    pub use crate::block::*;
    pub use crate::canvas::*;
    pub use bracket_lib::prelude::*;

    pub type Color = (u8, u8, u8);

    pub const SCREEN_WIDTH: i32 = 30;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    canvas: Canvas,
    active_block: Block,
}

impl State {
    fn new() -> Self {
        Self {
            canvas: Canvas::new(),
            active_block: Block::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.canvas.bake(self.active_block.as_pixels());
        }

        self.active_block.update(ctx);

        self.canvas.render(ctx);
        self.active_block.render(ctx);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Bracktrix")
        .build()?;
    main_loop(ctx, State::new())
}
