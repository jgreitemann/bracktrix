use crate::prelude::*;

#[system]
#[write_component(Position)]
#[write_component(Pivot)]
#[read_component(Active)]
pub fn player_input(
    world: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] scoring: &mut Scoring,
) {
    if let &Some(key) = key {
        use super::collision::*;
        use VirtualKeyCode::*;
        match key {
            Left => {
                apply_if_collision_free(world, Translation(Point::new(-1, 0)));
            }
            Right => {
                apply_if_collision_free(world, Translation(Point::new(1, 0)));
            }
            Up => {
                apply_if_collision_free(world, Rotation::Deg270);
            }
            Down => {
                apply_if_collision_free(world, Rotation::Deg90);
            }
            Space => {
                scoring.hard_drop();
            }
            _ => {}
        };
    }
}
