use crate::prelude::*;
use legion::query::{DefaultFilter, EntityFilter, View};

#[system]
#[write_component(Position)]
#[write_component(Pivot)]
#[read_component(Active)]
pub fn player_input(world: &mut SubWorld, #[resource] key: &Option<VirtualKeyCode>) {
    if let &Some(key) = key {
        let filter = component::<Active>();
        use VirtualKeyCode::*;
        match key {
            Left => try_apply_transform(world, filter, Translation(Point::new(-1, 0))),
            Right => try_apply_transform(world, filter, Translation(Point::new(1, 0))),
            Up => try_apply_transform(world, filter, Rotation::Deg270),
            Down => try_apply_transform(world, filter, Rotation::Deg90),
            _ => {}
        }
    }
}

fn try_apply_transform<'a, V, F, T>(world: &'a mut SubWorld, filter: F, transform: T)
where
    T::Element: IntoQuery<View = V>,
    V: View<'a, Element = T::Element> + DefaultFilter,
    F: EntityFilter,
    <V as DefaultFilter>::Filter: EntityFilter + std::ops::BitAnd + std::ops::BitAnd<F>,
    <<V as DefaultFilter>::Filter as std::ops::BitAnd<F>>::Output: EntityFilter,
    T: Transform<'a>,
{
    let mut query = <T::Element>::query().filter(filter);

    query.for_each_mut(world, |elem| {
        transform.apply_to(elem);
    });

    /*
    Rollback:
    let changed: Vec<_> = query
        .iter_mut(world)
        .map(|elem| transform.apply_to(elem))
        .take(2)
        .collect();

    changed.into_iter().for_each(|elem| {
        transform.inv().apply_to(elem);
    });
     */
}
