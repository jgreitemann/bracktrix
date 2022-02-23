use crate::prelude::*;

#[system]
#[read_component(ScoreboardItem)]
#[read_component(DisplayText)]
#[read_component(Score)]
pub fn scoreboard_render(world: &SubWorld, #[resource] scoring: &Scoring) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    <(&ScoreboardItem, &DisplayText, &Score)>::query().for_each(
        world,
        |(ScoreboardItem { rect }, DisplayText(text), &Score { metric, style })| {
            draw_batch.print(Point::new(rect.x1, rect.y1), text);
            match style {
                ScoreStyle::Text => {
                    draw_batch.print_right(Point::new(rect.x2, rect.y2), scoring.get_text(metric));
                }
                ScoreStyle::ProgressBar => {
                    draw_batch.bar_horizontal(
                        Point::new(rect.x1, rect.y2),
                        rect.width(),
                        (scoring.get_fraction(metric) * 100f32) as i32,
                        100,
                        ColorPair::new(WHITE, BLACK),
                    );
                }
            }
        },
    );

    draw_batch.submit(2500).expect("Batch error");
}
