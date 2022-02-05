mod block;

mod prelude {
    pub use crate::block::*;
    pub use bracket_lib::prelude::*;

    pub const SCREEN_WIDTH: u32 = 30;
    pub const SCREEN_HEIGHT: u32 = 50;
}

use prelude::*;

struct State {
    active_block: Block,
}

impl State {
    fn new() -> Self {
        Self {
            active_block: Block::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.active_block.update(ctx);
        self.active_block.render(ctx);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Bracktrix")
        .build()?;
    main_loop(ctx, State::new())
}
