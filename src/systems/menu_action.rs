use crate::prelude::*;

#[system(for_each)]
pub fn menu_action(entity: &Entity, action: &Action, cmd: &mut CommandBuffer) {
    let action_copy = action.clone();
    cmd.exec_mut(move |_world, resources| match action_copy {
        Action::NotImplemented => todo!(),
        Action::BackToMainMenu => {
            *resources.get_mut::<GameMode>().unwrap() = GameMode::Menu(Menu::Main);
        }
        Action::Quit => {
            *resources.get_mut::<GameMode>().unwrap() = GameMode::Quitting;
        }
    });

    cmd.remove(*entity);
}
