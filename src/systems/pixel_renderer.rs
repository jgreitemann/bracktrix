use crate::prelude::*;

#[system]
#[read_component(Pixels)]
#[read_component(Transform)]
#[read_component(Render)]
pub fn pixel_render(world: &SubWorld) {
    for (
        Pixels(pixels),
        &Transform {
            translation,
            rotation,
        },
        render,
    ) in <(&Pixels, &Transform, &Render)>::query().iter(world)
    {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(render.console);

        for pix in pixels {
            if let Some(screen_point) = to_screen(
                &(rotation.apply_to(&pix.position) + translation),
                &render.viewport,
            ) {
                draw_batch.set(
                    screen_point,
                    ColorPair::new(pix.color, BLACK),
                    to_cp437(pix.glyph),
                );
            }
        }

        draw_batch.submit(render.z_order).expect("Batch error");
    }
}
