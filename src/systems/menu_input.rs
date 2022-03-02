use crate::prelude::*;

#[system]
#[write_component(Focus)]
#[read_component(Active)]
pub fn menu_input(
    world: &mut SubWorld,
    #[state] menu_input_state: &mut MenuInputState,
    #[resource] input: &RawInputSignal,
) {
    menu_input_state.process(input);

    if let Some(menu_input) = menu_input_state.get() {
        if let Some(focus) = <&mut Focus>::query().iter_mut(world).next() {
            use MenuInput::*;
            match menu_input {
                NavigateUp => focus.up(),
                NavigateDown => focus.down(),
                Choose => todo!(),
            }
        }
    }
}
