use amethyst::{
    core::Transform,
    ecs::{Component, Entities, Join, ReadStorage, WriteStorage, System, DenseVecStorage},
    ecs::world::{Entity, Index}
};

use hibitset::BitSet;
use std::collections::HashMap;

use crate::math::{Vec2, Rect};

#[derive(Component)]
pub struct Collider {
    size: Vec2<f32>,
    /// The level with which the object is resisting change in position. When
    /// two objects collide, the one with the lower tenacity will move.
    /// If two objects have the same tenacity the collision is ignored.
    tenacity: u8
}

impl Collider {
    /// Create a new collider with optional tenacity. If no tenacity is given,
    /// the maximum value is used, making the object essentially immovable.
    pub fn new(size: Vec2<f32>, tenacity: Option<u8>) -> Collider {
        Collider {
            size,
            tenacity: tenacity.unwrap_or(u8::max_value())
        }
    }

    pub fn size(&self) -> Vec2<f32> { self.size }

    pub fn tenacity(&self) -> u8 { self.tenacity }
}

pub struct Collision;
impl<'s> System<'s> for Collision {
    type SystemData = (
        Entities<'s>,
        // TODO: Add a flogged storage for changed colliders
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (entities, colliders, mut transforms): Self::SystemData) {
        // Hibitset to mark which entities have to be moved and HashMap that saves,
        // where the moving entities have to be moved.
        let mut to_move = BitSet::new();
        let mut required_moves: HashMap<Index, Vec<Vec2<f32>>> = HashMap::new();

        // ? What happens when more than two collide?
        for (entity_1, collider_1, collider_trans_1) in (&entities, &colliders, &transforms).join() {
            for (entity_2, collider_2, collider_trans_2) in (&entities, &colliders, &transforms).join() {
                let collider_rect_1 = Rect::from_transform_as_middle(&collider_trans_1, collider_1.size());
                let collider_rect_2 = Rect::from_transform_as_middle(&collider_trans_2, collider_2.size());

                if Rect::intersect(&collider_rect_1, &collider_rect_2) {
                    // Check which entity has to be moved, if any
                    // TODO: This could be made nicer with an array of SystemData
                    // of two elements, or more confusing. One of the two.
                    let mut budging: Option<(Entity, Rect<f32>, Rect<f32>)> = None;
                    if collider_1.tenacity() < collider_2.tenacity() {
                        // The first collider yields
                        budging = Some((entity_1, collider_rect_1, collider_rect_2));
                    }
                    else if collider_2.tenacity() < collider_1.tenacity() {
                        // The second collider yields
                        budging = Some((entity_2, collider_rect_2, collider_rect_1));
                    }
                    // Otherwise nothing gets moved, since both objects have
                    // the same tenacity

                    // Mark the budging collider for movement if it has not been
                    // already marked and add the movements necessary for it.
                    if let Some((budging, budging_rect, reference_rect)) = budging {
                        if !to_move.add(budging.id()) {
                            required_moves.insert(budging.id(), vec![budging_rect.shortest_way_out(&reference_rect)]);
                        }
                        else {
                            required_moves.get_mut(&budging.id())
                                .expect("to_move and required_moves do not contain the same keys")
                                .push(budging_rect.shortest_way_out(&reference_rect));
                        }
                    }
                }
            }
        }

        // Go through all the colliders that have to be moved and actually move them
        for (to_move, transform) in (&to_move, &mut transforms).join() {
            for movement in required_moves.get(&to_move).expect("to_move and required_moves do not contain the same keys") {
                transform.prepend_translation_x(movement.x);
                transform.prepend_translation_y(movement.y);
            }
        }
    }
}