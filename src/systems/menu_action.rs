use crate::prelude::*;

#[system(for_each)]
pub fn menu_action(entity: &Entity, action: &Action, cmd: &mut CommandBuffer) {
    match action {
        Action::Print(text) => {
            println!("{}", text);
        }
    }

    cmd.remove(*entity);
}
