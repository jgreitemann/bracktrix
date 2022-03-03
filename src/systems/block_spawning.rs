use crate::prelude::*;

#[system]
#[read_component(Active)]
#[read_component(Preview)]
pub fn block_spawning(
    world: &SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] spawn_points: &BlockSpawnPoints,
    #[resource] rng: &mut RandomNumberGenerator,
) {
    if <&Preview>::query().iter(world).next().is_none() {
        cmd.extend(Block::random(rng).components::<Preview>(&spawn_points.preview_block_spawn));
    }

    if <&Active>::query().iter(world).next().is_none() {
        <Entity>::query()
            .filter(component::<Preview>())
            .for_each(world, |&entity| {
                cmd.remove_component::<Preview>(entity);
                cmd.add_component(entity, Active {});
            });

        let translation =
            Translation(spawn_points.active_block_spawn - spawn_points.preview_block_spawn);
        cmd.exec_mut(move |world, resources| {
            let (mut subworld, _) = world.split::<(&mut Position, &mut Pivot, &Active)>();
            if !super::collision::apply_if_collision_free(&mut subworld, translation) {
                *resources.get_mut::<GameMode>().unwrap() = GameMode::Menu(Menu::GameOver);
            }
        });
    }
}
