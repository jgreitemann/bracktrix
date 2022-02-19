use crate::prelude::*;

#[system]
#[read_component(Position)]
#[read_component(PixelRender)]
pub fn pixel_render(world: &SubWorld, #[resource] Screen(screen_rect): &Screen) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    <(&Position, &PixelRender)>::query()
        .iter(world)
        .filter(|(&Position(pt), _)| screen_rect.point_in_rect(pt))
        .for_each(|(&Position(pt), &PixelRender { colors, glyph })| {
            draw_batch.set(pt, colors, glyph);
        });

    draw_batch.submit(0).expect("Batch error");
}
