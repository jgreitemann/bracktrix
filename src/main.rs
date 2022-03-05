mod block;
mod components;
mod graphics;
mod input;
mod menu;
mod resources;
mod scaffold;
mod scoring;
mod systems;

#[cfg(test)]
mod test_utils;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::{Builder, CommandBuffer};
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub use crate::block::*;
    pub use crate::components::*;
    pub use crate::graphics::*;
    pub use crate::input::*;
    pub use crate::resources::*;
    pub use crate::scaffold::*;
    pub use crate::scoring::*;
    pub use crate::systems::*;

    pub const SCREEN_WIDTH: usize = 23;
    pub const SCREEN_HEIGHT: usize = 25;
    pub const CANVAS_WIDTH: usize = 12;
    pub const CANVAS_HEIGHT: usize = 21;
    pub const SCALE: usize = 3;
    pub const TEXT_SCALE: usize = 2;
}

use crate::menu::{MenuBuilder, ScoreboardBuilder};
use prelude::*;

struct State {
    world: World,
    input_sources: Vec<Box<dyn InputSource>>,
    resources: Resources,
    base_systems: Schedule,
    play_systems: Schedule,
    menu_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut world = World::default();
        let mut resources = Resources::default();

        resources.insert(GameMode::Menu(Menu::Main));
        resources.insert(RandomNumberGenerator::new());
        resources.insert(Scoring::default());

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

        MenuBuilder::new(Menu::Main, &world)
            .add_text("Welcome to Bracktrix")
            .add_text("~~~~~~~~")
            .add_button("Play Game", Action::StartGame)
            .add_button("Show Leaderboard", Action::NotImplemented)
            .add_button("Quit", Action::Quit)
            .build(&mut world, &mut resources);

        MenuBuilder::new(Menu::GameOver, &world)
            .add_text("Game Over!")
            .add_score("Reached level:", Metric::Level)
            .add_score("Final score:", Metric::Score)
            .add_text("~~~~~~~~")
            .add_button("Play Again", Action::StartGame)
            .add_button("Show Leaderboard", Action::NotImplemented)
            .add_button("More Statistics", Action::GoToMenu(Menu::Statistics))
            .add_button("Back to Main Menu", Action::GoToMenu(Menu::Main))
            .build(&mut world, &mut resources);

        MenuBuilder::new(Menu::Statistics, &world)
            .add_text("Game Statistics")
            .add_text("~~~~~~~~~~~~~~~~~")
            .add_score("Reached level:", Metric::Level)
            .add_score("Final score:", Metric::Score)
            .add_score("Lines cleared:", Metric::LinesCleared)
            .add_score("# Bracktrixes:", Metric::NumberOfBracktrixes)
            .add_score("Time elapsed:", Metric::TimeElapsed)
            .add_score("Blocks placed:", Metric::BlocksPlaced)
            .add_button("Back", Action::GoToMenu(Menu::GameOver))
            .build(&mut world, &mut resources);

        MenuBuilder::new(Menu::Pause, &world)
            .add_text("Game Paused")
            .add_text("~~~~~~~~")
            .add_button("Resume Game", Action::ResumeGame)
            .add_button("Restart Game", Action::StartGame)
            .add_button("Back to Main Menu", Action::GoToMenu(Menu::Main))
            .build(&mut world, &mut resources);

        ScoreboardBuilder::new(&world, &mut scaffold.score_rects())
            .add_score("Level:", Metric::Level)
            .add_progress_bar("Level up:", Metric::LevelUpFraction)
            .add_score("Score:", Metric::Score)
            .add_score("Lines cleared:", Metric::LinesCleared)
            .add_score("# Bracktrixes:", Metric::NumberOfBracktrixes)
            .add_score("Time elapsed:", Metric::TimeElapsed)
            .add_score("Blocks placed:", Metric::BlocksPlaced)
            .build(&mut world, &mut resources);

        Self {
            world,
            input_sources: vec![
                Box::new(KeyboardInputSource),
                Box::new(GamepadInputSource::new()),
            ],
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
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls_bg((0, 0, 0, 0));

        let input = self
            .input_sources
            .iter_mut()
            .fold(RawInputSignal::default(), |prev, source| {
                prev.or(source.read())
            });
        self.resources.insert(input);

        let mode = *self.resources.get::<GameMode>().unwrap();
        self.base_systems
            .execute(&mut self.world, &mut self.resources);
        match mode {
            GameMode::Play => &mut self.play_systems,
            GameMode::Menu(menu) => {
                self.resources.insert(menu);
                &mut self.menu_systems
            }
            GameMode::Quitting => {
                ctx.quit();
                return;
            }
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
        .with_simple_console_no_bg(
            TEXT_SCALE * SCREEN_WIDTH,
            TEXT_SCALE * SCREEN_HEIGHT,
            "terminal8x8.png",
        )
        .with_simple_console(
            TEXT_SCALE * SCREEN_WIDTH,
            TEXT_SCALE * SCREEN_HEIGHT,
            "terminal8x8.png",
        )
        .build()?;
    main_loop(ctx, State::new())
}
