use crate::prelude::*;
use legion::systems::CommandBuffer;

#[system]
pub fn block_spawning(
    cmd: &mut CommandBuffer,
    #[resource] store: &mut BlockEntityStore,
    #[resource] spawn_points: &BlockSpawnPoints,
) {
    if store.preview.is_none() {
        let block = BlockShape::random();
        let spawn = spawn_points.preview_block_spawn;
        let offset = block.rotation_offset();
        store.preview = Some(
            <[Entity; 4]>::try_from(cmd.extend(block.pixels().map(move |pix| {
                (
                    Preview {},
                    Position(pix.position + spawn),
                    Pivot(pix.position * 2 - offset),
                    PixelRender {
                        colors: ColorPair::new(pix.color, BLACK),
                        glyph: to_cp437(pix.glyph),
                    },
                )
            })))
            .unwrap(),
        );
    }

    if store.active.is_none() {
        if let Some(preview_entities) = std::mem::replace(&mut store.preview, None) {
            for entity in preview_entities {
                cmd.remove_component::<Preview>(entity);
                cmd.add_component(entity, Active {});
            }
            let translation =
                Translation(spawn_points.active_block_spawn - spawn_points.preview_block_spawn);
            cmd.exec_mut(move |world, _| {
                super::collision::transform_active_entities(world, &translation);
            });
            store.active = Some(preview_entities);
        }
    }
}
