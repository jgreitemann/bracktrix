use crate::prelude::*;

const BLOCK_GLYPHS: [char; 16] = [
    '█', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓',
];

#[system]
#[write_component(Flagged)]
#[write_component(PixelRender)]
pub fn line_deletion(world: &mut SubWorld, cmd: &mut CommandBuffer) {
    <(Entity, &mut Flagged, &mut PixelRender)>::query().for_each_mut(
        world,
        |(&entity, flag, render)| {
            flag.frames_till_death -= 1;
            if flag.frames_till_death == 0 {
                cmd.remove(entity);
            } else {
                render.glyph = to_cp437(BLOCK_GLYPHS[flag.frames_till_death % BLOCK_GLYPHS.len()]);
            }
        },
    )
}
