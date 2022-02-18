use crate::prelude::*;

#[system]
#[write_component(Position)]
#[write_component(Pivot)]
#[read_component(Active)]
pub fn gravity(
    world: &mut SubWorld,
    #[state] frame_index: &mut usize,
    #[resource] difficulty: &Difficulty,
) {
    *frame_index += 1;
    if *frame_index % difficulty.gravity_tick_speed == 0 {
        super::collision::apply_if_collision_free(world, Translation(Point::new(0, 1)));
    }
}
