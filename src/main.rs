mod block;
mod components;
mod graphics;
mod input;
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

        resources.insert(GameMode::Play);
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

        let mut scoreboard_rect_iter = scaffold.score_rects();
        world.push((MenuItem { rank: 0 }, DisplayText("Game Over!".to_string())));
        world.push((
            MenuItem { rank: 1 },
            ScoreboardItem {
                rect: scoreboard_rect_iter.next().unwrap(),
            },
            DisplayText("Level:".to_string()),
            Score {
                metric: Metric::Level,
                style: ScoreStyle::Text,
            },
        ));
        world.push((
            ScoreboardItem {
                rect: scoreboard_rect_iter.next().unwrap(),
            },
            DisplayText("Level up:".to_string()),
            Score {
                metric: Metric::LevelUpFraction,
                style: ScoreStyle::ProgressBar,
            },
        ));
        world.push((
            MenuItem { rank: 2 },
            ScoreboardItem {
                rect: scoreboard_rect_iter.next().unwrap(),
            },
            DisplayText("Score:".to_string()),
            Score {
                metric: Metric::Score,
                style: ScoreStyle::Text,
            },
        ));
        world.push((
            ScoreboardItem {
                rect: scoreboard_rect_iter.next().unwrap(),
            },
            DisplayText("Lines cleared:".to_string()),
            Score {
                metric: Metric::LinesCleared,
                style: ScoreStyle::Text,
            },
        ));
        world.push((
            ScoreboardItem {
                rect: scoreboard_rect_iter.next().unwrap(),
            },
            DisplayText("# Bracktrixes:".to_string()),
            Score {
                metric: Metric::NumberOfBracktrixes,
                style: ScoreStyle::Text,
            },
        ));
        world.push((
            ScoreboardItem {
                rect: scoreboard_rect_iter.next().unwrap(),
            },
            DisplayText("Time elapsed:".to_string()),
            Score {
                metric: Metric::TimeElapsed,
                style: ScoreStyle::Text,
            },
        ));
        world.push((
            ScoreboardItem {
                rect: scoreboard_rect_iter.next().unwrap(),
            },
            DisplayText("Blocks placed:".to_string()),
            Score {
                metric: Metric::BlocksPlaced,
                style: ScoreStyle::Text,
            },
        ));

        let input_sources: Vec<Box<dyn InputSource>> = vec![
            Box::new(KeyboardInputSource),
            Box::new(GamepadInputSource::new()),
        ];

        Self {
            world,
            input_sources,
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
