use crate::prelude::*;

#[system]
#[read_component(Position)]
#[read_component(PixelRender)]
#[read_component(NewViewport)]
pub fn pixel_render(world: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    <(&Position, &PixelRender, &NewViewport)>::query()
        .iter(world)
        .filter_map(|(Position(pos), render, NewViewport(viewport_rect))| {
            to_screen(&pos, &viewport_rect).map(|screen_point| (screen_point, render))
        })
        .for_each(|(screen_point, &PixelRender { colors, glyph })| {
            draw_batch.set(screen_point, colors, glyph);
        });

    draw_batch.submit(0).expect("Batch error");
}
