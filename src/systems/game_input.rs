use crate::prelude::*;

#[system]
#[write_component(Position)]
#[write_component(Pivot)]
#[read_component(Active)]
pub fn game_input(
    world: &mut SubWorld,
    #[state] game_input_state: &mut GameInputState,
    #[resource] input: &RawInputSignal,
    #[resource] scoring: &mut Scoring,
) {
    game_input_state.process(input);

    if let Some(game_input) = game_input_state.get() {
        use super::collision::*;
        use GameInput::*;
        match game_input {
            ShiftLeft => {
                apply_if_collision_free(world, Translation(Point::new(-1, 0)));
            }
            ShiftRight => {
                apply_if_collision_free(world, Translation(Point::new(1, 0)));
            }
            RotateCCW => {
                apply_if_collision_free(world, Rotation::Deg270);
            }
            RotateCW => {
                apply_if_collision_free(world, Rotation::Deg90);
            }
            HardDrop => {
                scoring.hard_drop();
            }
            SoftDrop => {
                scoring.soft_drop(true);
            }
        };
    } else {
        scoring.soft_drop(false);
    }
}
