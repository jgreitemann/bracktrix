use crate::prelude::*;
use legion::systems::CommandBuffer;

#[system]
#[write_component(Position)]
#[write_component(Pivot)]
#[read_component(Active)]
pub fn gravity(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    #[state] frame_index: &mut usize,
    #[resource] difficulty: &Difficulty,
) {
    *frame_index += 1;
    if *frame_index % difficulty.gravity_tick_speed == 0 {
        if !super::collision::apply_if_collision_free(world, Translation(Point::new(0, 1))) {
            <Entity>::query()
                .filter(component::<Active>())
                .for_each(world, |&entity| {
                    cmd.remove_component::<Active>(entity);
                })
        }
    }
}
