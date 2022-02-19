use crate::prelude::*;

#[system]
#[read_component(Active)]
#[read_component(Preview)]
pub fn block_spawning(
    world: &SubWorld,
    cmd: &mut CommandBuffer,
    #[resource] spawn_points: &BlockSpawnPoints,
) {
    if <&Preview>::query().iter(world).next().is_none() {
        let block = BlockShape::random();
        let spawn = spawn_points.preview_block_spawn;
        let offset = block.rotation_offset();
        cmd.extend(block.pixels().map(move |pix| {
            (
                Preview {},
                Position(pix.position + spawn),
                Pivot(pix.position * 2 - offset),
                PixelRender {
                    colors: ColorPair::new(pix.color, BLACK),
                    glyph: to_cp437(pix.glyph),
                },
            )
        }));
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
        cmd.exec_mut(move |world, _| {
            super::collision::transform_active_entities(world, &translation);
        });
    }
}
