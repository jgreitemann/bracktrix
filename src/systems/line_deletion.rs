use crate::prelude::*;
use std::collections::HashSet;

const ANIMATION_GLYPHS: [char; 16] = [
    '█', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓', '▒', '░', '▒', '▓',
];

#[system]
#[write_component(Flagged)]
#[write_component(PixelRender)]
#[write_component(Position)]
#[read_component(Settled)]
pub fn line_deletion(world: &mut SubWorld, cmd: &mut CommandBuffer) {
    let deleted_lines: HashSet<_> = <(Entity, &mut Flagged, &mut PixelRender, &Position)>::query()
        .iter_mut(world)
        .filter_map(|(&entity, flag, render, Position(Point { y, .. }))| {
            flag.frames_till_death += 1;
            if flag.frames_till_death == ANIMATION_GLYPHS.len() {
                cmd.remove(entity);
                Some(*y)
            } else {
                render.glyph = to_cp437(ANIMATION_GLYPHS[flag.frames_till_death]);
                None
            }
        })
        .collect();

    // Drop settled blocks down
    if !deleted_lines.is_empty() {
        let y_shifts: Vec<_> = (0..SCREEN_HEIGHT as i32)
            .map(|y| deleted_lines.iter().filter(|&&del_y| del_y > y).count() as i32)
            .collect();
        <&mut Position>::query()
            .filter(component::<Settled>())
            .for_each_mut(world, |Position(Point { y, .. })| {
                *y += y_shifts[*y as usize]
            });
    }
}
