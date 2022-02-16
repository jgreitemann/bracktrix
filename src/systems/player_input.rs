use crate::prelude::*;

#[system]
#[write_component(Position)]
#[write_component(Pivot)]
pub fn player_input(world: &mut SubWorld, #[resource] key: &Option<VirtualKeyCode>) {
    if let &Some(key) = key {
        match key {
            VirtualKeyCode::Left => apply_translation(world, Point::new(-1, 0)),
            VirtualKeyCode::Right => apply_translation(world, Point::new(1, 0)),
            VirtualKeyCode::Up => apply_rotation(world, Rotation::Deg90),
            VirtualKeyCode::Down => apply_rotation(world, Rotation::Deg270),
            _ => {}
        }
    }
}

fn apply_translation(world: &mut SubWorld, delta: Point) {
    for Position(pos) in <&mut Position>::query().iter_mut(world) {
        *pos += delta;
    }
}

fn apply_rotation(world: &mut SubWorld, rotation: Rotation) {
    for (Position(pos), pivot) in <(&mut Position, &mut Pivot)>::query().iter_mut(world) {
        let new_pivot = rotation.apply_to(&pivot.point);
        *pos += new_pivot - pivot.point;
        pivot.point = new_pivot;
    }
}
