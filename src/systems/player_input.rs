use crate::prelude::*;

#[system]
#[write_component(Transform)]
pub fn player_input(world: &mut SubWorld, #[resource] key: &Option<VirtualKeyCode>) {
    if let &Some(key) = key {
        for transform in <(&mut Transform)>::query().iter_mut(world) {
            *transform = match key {
                VirtualKeyCode::Left => transform.shifted_by(&Point::new(-1, 0)),
                VirtualKeyCode::Right => transform.shifted_by(&Point::new(1, 0)),
                VirtualKeyCode::Up => transform.rotate_counter_clockwise(),
                VirtualKeyCode::Down => transform.rotate_clockwise(),
                _ => *transform,
            };
        }
    }
}
