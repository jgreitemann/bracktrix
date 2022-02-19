use crate::prelude::*;
use itertools::Itertools;

const SPACING: usize = 3;
const H_PADDING: usize = 3;
const V_PADDING: usize = 2;

#[system]
#[read_component(MenuItem)]
#[read_component(DisplayText)]
#[read_component(Metric)]
pub fn menu_render(world: &SubWorld, #[resource] scoring: &Scoring) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.cls_color((0, 0, 0, 200));

    let entries = <(&MenuItem, &DisplayText, Option<&Metric>)>::query()
        .iter(world)
        .sorted_by_key(|(MenuItem { rank }, ..)| rank)
        .map(|(_, DisplayText(text), stat)| match stat {
            Some(&metric) => format!("{} {}", text, scoring.get(metric)),
            None => text.clone(),
        })
        .collect_vec();

    if let Some(max_len) = entries.iter().map(|text| text.len()).max() {
        let menu_width = (max_len / 2 + H_PADDING) * 2;
        let menu_height = SPACING * (entries.len() - 1) + 2 * V_PADDING;

        let menu_rect = Rect::with_size(
            SCREEN_WIDTH - menu_width / 2,
            SCREEN_HEIGHT - menu_height / 2,
            menu_width,
            menu_height,
        );

        draw_batch.draw_box(menu_rect, ColorPair::new(WHITE, BLACK));

        entries.iter().enumerate().for_each(|(i, text)| {
            draw_batch.print_centered(menu_rect.y1 + (V_PADDING + SPACING * i) as i32, text);
        });
    }

    draw_batch.submit(5000).expect("Batch error");
}
