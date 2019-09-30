use amethyst::{
    core::Transform,
    ecs::{Component, Join, ReadStorage, WriteStorage, System, DenseVecStorage}
};

use std::collections::HashMap;

use crate::math::{Vec2, Rect};
use crate::player::Player;

#[derive(Component)]
pub struct Collider {
    size: Vec2<f32>,
}

impl Collider {
    pub fn size(&self) -> Vec2<f32> {
        self.size
    }
}

pub struct Collision;
impl<'s> System<'s> for Collision {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Collider>,
        WriteStorage<'s, Transform>
    );

    fn run(&mut self, (player, colliders, mut transforms): Self::SystemData) {
        // Get the player transform immutably.
        // XXX: This assumes that there can only be one player and therefore only

        let player_rect = Rect::from_transform_as_middle(&transforms.get(&player).expect("Player has no transform"), player.size());
        for (collider, collider_trans) in (&colliders, &transforms).join() {
            let collider_rect = Rect::from_transform_as_middle(&collider_trans, collider.size());

            if Rect::intersect(&player_rect, &collider_rect) {
                let way_out = player_rect.shortest_way_out(&collider_rect);
                player_trans.prepend_translation_x(way_out.x);
                player_trans.prepend_translation_y(way_out.y);
            }
        }


        transform.get_mut(&player)
    }
}