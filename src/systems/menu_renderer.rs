use crate::prelude::*;

#[system]
#[read_component(DisplayText)]
pub fn menu_render(world: &SubWorld) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    draw_batch.cls_color((0, 0, 0, 200));

    <&DisplayText>::query().for_each(world, |DisplayText(text)| {
        draw_batch.print_centered(25, text);
    });

    draw_batch.submit(5000).expect("Batch error");
}
