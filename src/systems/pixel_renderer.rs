use crate::prelude::*;

#[system]
#[read_component(Position)]
#[read_component(PixelRender)]
pub fn pixel_render(world: &SubWorld, #[resource] Screen(screen_rect): &Screen) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    <(&Position, &PixelRender)>::query()
        .iter(world)
        .filter_map(|(Position(pos), render)| {
            to_screen(&pos, &screen_rect).map(|screen_point| (screen_point, render))
        })
        .for_each(|(screen_point, &PixelRender { colors, glyph })| {
            draw_batch.set(screen_point, colors, glyph);
        });

    draw_batch.submit(0).expect("Batch error");
}
