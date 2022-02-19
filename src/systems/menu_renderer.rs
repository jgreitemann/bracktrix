use crate::prelude::*;

#[system]
pub fn menu_render() {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    draw_batch.cls_color((0, 0, 0, 200));

    draw_batch.print_centered(25, "Game Over!");

    draw_batch.submit(5000).expect("Batch error");
}
