use crate::prelude::*;

pub fn apply_if_collision_free(world: &mut SubWorld, transform: impl Transform) {
    let moved: Vec<_> = <(&mut Position, &mut Pivot)>::query()
        .filter(component::<Active>())
        .iter_mut(world)
        .map(|(pos, pivot)| {
            transform.apply_to(pos, pivot);
            *pos
        })
        .collect();

    let did_collide = <&Position>::query()
        .filter(!component::<Active>())
        .iter(world)
        .any(|pos| moved.contains(pos));

    if did_collide {
        let inverse = transform.inv();
        <(&mut Position, &mut Pivot)>::query()
            .filter(component::<Active>())
            .for_each_mut(world, |(pos, pivot)| inverse.apply_to(pos, pivot));
    }
}
