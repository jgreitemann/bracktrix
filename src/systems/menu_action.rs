use crate::prelude::*;

#[system(for_each)]
pub fn menu_action(entity: &Entity, action: &Action, cmd: &mut CommandBuffer) {
    let action_copy = action.clone();
    cmd.exec_mut(move |world, resources| match action_copy {
        Action::NotImplemented => todo!(),
        Action::StartGame => {
            let game_entities: Vec<_> = <Entity>::query()
                .filter(component::<Game>())
                .iter(world)
                .cloned()
                .collect();
            for entity in game_entities {
                world.remove(entity);
            }
            resources.insert(Scoring::default());
            resources.insert(GameMode::Play);
        }
        Action::ResumeGame => resources.insert(GameMode::Play),
        Action::GoToMenu(menu) => resources.insert(GameMode::Menu(menu)),
        Action::Quit => resources.insert(GameMode::Quitting),
    });

    cmd.remove(*entity);
}
