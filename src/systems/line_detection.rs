use crate::prelude::*;

#[system]
#[read_component(Settled)]
#[read_component(Position)]
pub fn line_detection(world: &SubWorld, cmd: &mut CommandBuffer) {
    use itertools::Itertools;
    <(Entity, &Position)>::query()
        .filter(component::<Settled>() & !component::<Flagged>())
        .iter(world)
        .map(|(entity, Position(Point { y, .. }))| (y, entity))
        .into_group_map()
        .into_values()
        .filter(|line| line.len() >= CANVAS_WIDTH)
        .flatten()
        .for_each(|&entity| cmd.add_component(entity, Flagged::new()));
}
