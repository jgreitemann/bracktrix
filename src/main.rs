use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(8, "Hello, world!");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50().with_title("Bracktrix").build()?;
    main_loop(ctx, State{})
}
