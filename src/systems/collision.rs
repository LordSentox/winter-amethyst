use amethyst::{
    core::Transform,
    ecs::{Component, ReadStorage, WriteStorage, System, VecStorage},
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

    fn run(&mut self, (mut player, colliders): Self::SystemData) {
        
    }
}