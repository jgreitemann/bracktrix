use crate::prelude::*;

#[system]
#[write_component(Position)]
#[write_component(Pivot)]
#[read_component(Active)]
pub fn gravity(world: &mut SubWorld, #[resource] frame: &Frame) {
    if frame.index % 4 == 0 {
        super::collision::apply_if_collision_free(world, Translation(Point::new(0, 1)));
    }
}
