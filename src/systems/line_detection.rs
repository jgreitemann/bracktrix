use crate::prelude::*;
use crate::CANVAS_WIDTH;
use itertools::Itertools;
use legion::systems::CommandBuffer;

#[system]
#[read_component(Settled)]
#[read_component(Position)]
pub fn line_detection(world: &SubWorld, cmd: &mut CommandBuffer) {
    use itertools::Itertools;
    <(Entity, &Position)>::query()
        .filter(component::<Settled>())
        .iter(world)
        .map(|(entity, Position(Point { y, .. }))| (y, entity))
        .into_group_map()
        .into_values()
        .filter(|line| line.len() == CANVAS_WIDTH)
        .flatten()
        .for_each(|&entity| cmd.add_component(entity, Flagged::new()));

    // TODO: move into own system
    cmd.exec_mut(|world, _| {
        <&mut PixelRender>::query()
            .filter(component::<Flagged>())
            .for_each_mut(world, |PixelRender { glyph, .. }| *glyph = to_cp437('░'));
    })
}
