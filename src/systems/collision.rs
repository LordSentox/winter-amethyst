use amethyst::{
    core::Transform,
    ecs::{Component, Join, ReadStorage, WriteStorage, System, VecStorage},
};

use crate::math::Rect;
use crate::player::Player;

pub type CollisionRect = Rect<f32>;

pub struct Collision;

impl Component for CollisionRect {
    type Storage = VecStorage<Self>;
}

impl<'s> System<'s> for Collision {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadStorage<'s, CollisionRect>
    );

    fn run(&mut self, (mut players, colliders): Self::SystemData) {
        // Let all players collide with all colliders
        for player in (&mut players).join() {
            for collider in (&colliders).join() {
                if CollisionRect::intersect(player.collision_rect(), collider) {
                    let way_out = player.collision_rect().shortest_way_out(collider);
                    player.translate(way_out);
                }
            }
        }
    }
}