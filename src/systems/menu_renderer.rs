use crate::prelude::*;
use itertools::Itertools;

const SPACING: usize = 3;
const H_PADDING: usize = 3;
const V_PADDING: usize = 2;

#[system]
#[read_component(MenuItem)]
#[read_component(DisplayText)]
#[read_component(Score)]
#[read_component(Focus)]
pub fn menu_render(world: &SubWorld, #[resource] scoring: &Scoring) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.cls_color((0, 0, 0, 200));

    let entries = <(&MenuItem, &DisplayText, Option<&Score>, Option<&Focus>)>::query()
        .iter(world)
        .sorted_by_key(|(MenuItem { rank }, ..)| rank)
        .map(|(_, DisplayText(text), stat, focus)| {
            (
                match stat {
                    Some(&Score { metric, .. }) => format!("{} {}", text, scoring.get_text(metric)),
                    None => text.clone(),
                },
                focus.is_some(),
            )
        })
        .collect_vec();

    if let Some(max_len) = entries.iter().map(|(text, _)| text.chars().count()).max() {
        let menu_width = (max_len / 2 + H_PADDING + 3) * 2;
        let menu_height = SPACING * entries.len() + 2 * V_PADDING - 1;

        let menu_rect = Rect::with_size(
            SCREEN_WIDTH - menu_width / 2,
            SCREEN_HEIGHT - menu_height / 2,
            menu_width,
            menu_height,
        );

        draw_batch.draw_double_box(menu_rect, ColorPair::new(WHITE, BLACK));

        let menu_rects = (0..entries.len()).map(|i| {
            Rect::with_size(
                menu_rect.x1 as usize + H_PADDING,
                menu_rect.y1 as usize + (V_PADDING + SPACING * i),
                menu_width - 2 * H_PADDING,
                SPACING - 1,
            )
        });

        entries
            .iter()
            .zip(menu_rects)
            .for_each(|((text, has_focus), rect)| {
                if *has_focus {
                    draw_batch.draw_box(rect, ColorPair::new(WHITE, BLACK));
                }
                let print_y = rect.center().y;
                draw_batch.print_centered(print_y, text);
            });
    }

    draw_batch.submit(5000).expect("Batch error");
}
