use crate::prelude::*;

#[system]
#[read_component(ScoreboardItem)]
#[read_component(DisplayText)]
#[read_component(Metric)]
pub fn scoreboard_render(world: &SubWorld, #[resource] scoring: &Scoring) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    <(&ScoreboardItem, &DisplayText, &Metric)>::query().for_each(
        world,
        |(ScoreboardItem { rect }, DisplayText(text), &metric)| {
            draw_batch.print(Point::new(rect.x1, rect.y1), text);
            draw_batch.print_right(Point::new(rect.x2, rect.y2), scoring.get(metric));
        },
    );

    draw_batch.submit(2500).expect("Batch error");
}
