use crate::prelude::*;

#[system]
#[write_component(Position)]
#[read_component(Active)]
pub fn gravity(world: &mut SubWorld, #[resource] frame: &Frame) {
    if frame.index % 4 == 0 {
        for Position(pos) in <&mut Position>::query()
            .filter(component::<Active>())
            .iter_mut(world)
        {
            pos.y += 1;
        }
    }
}
