use crate::prelude::*;
use itertools::Itertools;

#[system]
#[read_component(Actionable)]
#[read_component(MenuItem)]
#[read_component(Focus)]
pub fn menu_input(
    world: &mut SubWorld,
    cmd: &mut CommandBuffer,
    #[state] menu_input_state: &mut MenuInputState,
    #[resource] &active_menu: &Menu,
    #[resource] input: &RawInputSignal,
) {
    menu_input_state.process(input);

    if let Some(menu_input) = menu_input_state.get() {
        let entries: Vec<_> = <(Entity, &MenuItem, &Actionable, Option<&Focus>)>::query()
            .iter(world)
            .filter(|(_, &MenuItem { menu, .. }, ..)| menu == active_menu)
            .sorted_by_key(|(_, &MenuItem { rank, .. }, ..)| rank)
            .collect();

        if let Some((focus_index, &(focus_entity, _, Actionable(focus_action), _))) = entries
            .iter()
            .enumerate()
            .find(|(_, (_, _, _, f))| f.is_some())
        {
            use MenuInput::*;
            match menu_input {
                NavigateUp => {
                    cmd.remove_component::<Focus>(*focus_entity);
                    cmd.add_component(
                        *entries[(focus_index + entries.len() - 1) % entries.len()].0,
                        Focus,
                    );
                }
                NavigateDown => {
                    cmd.remove_component::<Focus>(*focus_entity);
                    cmd.add_component(*entries[(focus_index + 1) % entries.len()].0, Focus);
                }
                Choose => {
                    cmd.push((focus_action.clone(),));
                }
            }
        }
    }
}
