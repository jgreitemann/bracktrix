use crate::prelude::*;

pub struct MenuBuilder {
    rank: usize,
    focus: Option<Focus>,
    cmd: CommandBuffer,
}

impl MenuBuilder {
    pub fn new(world: &World) -> Self {
        Self {
            rank: 0,
            focus: Some(Focus),
            cmd: CommandBuffer::new(world),
        }
    }

    pub fn add_text<T: ToString>(mut self, text: T) -> Self {
        let item = self.make_item();
        self.cmd.push((item, DisplayText(text.to_string())));
        self
    }

    pub fn add_score<T: ToString>(mut self, label: T, metric: Metric) -> Self {
        let item = self.make_item();
        self.cmd.push((
            item,
            DisplayText(label.to_string()),
            Score {
                metric,
                style: ScoreStyle::Text,
            },
        ));
        self
    }

    pub fn add_button<T: ToString>(mut self, label: T) -> Self {
        let item = self.make_item();
        let button = self.cmd.push((
            item,
            DisplayText(label.to_string()),
            Actionable(Action::Print("Hello world".to_string())),
        ));
        if let Some(focus) = std::mem::replace(&mut self.focus, None) {
            self.cmd.add_component(button, focus);
        }
        self
    }

    fn make_item(&mut self) -> MenuItem {
        let new_rank = self.rank + 1;
        MenuItem {
            rank: std::mem::replace(&mut self.rank, new_rank),
        }
    }

    pub fn build(mut self, world: &mut World, resources: &mut Resources) {
        self.cmd.flush(world, resources);
    }
}

pub struct ScoreboardBuilder<'a> {
    rect_generator: &'a mut dyn Iterator<Item = Rect>,
    cmd: CommandBuffer,
}

impl<'a> ScoreboardBuilder<'a> {
    pub fn new(world: &World, rect_generator: &'a mut dyn Iterator<Item = Rect>) -> Self {
        Self {
            rect_generator,
            cmd: CommandBuffer::new(world),
        }
    }

    pub fn add_score<T: ToString>(mut self, label: T, metric: Metric) -> Self {
        let rect = self.rect_generator.next().unwrap();
        self.cmd.push((
            ScoreboardItem { rect },
            DisplayText(label.to_string()),
            Score {
                metric,
                style: ScoreStyle::Text,
            },
        ));
        self
    }

    pub fn add_progress_bar<T: ToString>(mut self, label: T, metric: Metric) -> Self {
        let rect = self.rect_generator.next().unwrap();
        self.cmd.push((
            ScoreboardItem { rect },
            DisplayText(label.to_string()),
            Score {
                metric,
                style: ScoreStyle::ProgressBar,
            },
        ));
        self
    }

    pub fn build(mut self, world: &mut World, resources: &mut Resources) {
        self.cmd.flush(world, resources);
    }
}
