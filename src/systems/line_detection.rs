use crate::prelude::*;

#[system]
#[read_component(Settled)]
#[read_component(Position)]
pub fn line_detection(
    world: &SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] scoring: &mut Scoring,
) {
    use itertools::Itertools;
    let completed_lines = <(Entity, &Position)>::query()
        .filter(component::<Settled>() & !component::<Flagged>())
        .iter(world)
        .map(|(entity, Position(Point { y, .. }))| (y, entity))
        .into_group_map()
        .into_values()
        .filter(|line| line.len() >= CANVAS_WIDTH)
        .collect_vec();

    scoring.score_lines_cleared(completed_lines.len());

    completed_lines
        .into_iter()
        .flatten()
        .for_each(|&entity| cmd.add_component(entity, Flagged::default()));
}
