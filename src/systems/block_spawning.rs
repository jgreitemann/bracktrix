use crate::prelude::*;
use legion::systems::CommandBuffer;

#[system]
pub fn block_spawning(
    cmd: &mut CommandBuffer,
    #[resource] store: &mut BlockEntityStore,
    #[resource] spawn_points: &BlockSpawnPoints,
) {
    if store.preview.is_empty() {
        let block = BlockShape::random();
        let spawn = spawn_points.preview_block_spawn;
        let offset = block.rotation_offset();
        store.preview = cmd
            .extend(block.pixels().into_iter().map(move |pix| {
                (
                    Preview {},
                    Position(pix.position + spawn),
                    Pivot(pix.position * 2 - offset),
                    PixelRender {
                        colors: ColorPair::new(pix.color, BLACK),
                        glyph: to_cp437(pix.glyph),
                    },
                )
            }))
            .to_vec();
    }

    if store.active.is_empty() {
        for entity in &store.preview {
            cmd.remove_component::<Preview>(*entity);
            cmd.add_component(*entity, Active {});
        }
        let translation =
            Translation(spawn_points.active_block_spawn - spawn_points.preview_block_spawn);
        cmd.exec_mut(move |world, _| {
            super::collision::transform_active_entities(world, &translation);
        });
        store.active = std::mem::replace(&mut store.preview, Vec::new());
    }
}
