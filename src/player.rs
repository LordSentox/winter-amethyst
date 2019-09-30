use amethyst::{
    ecs::{Component, DenseVecStorage}
};

use crate::math::Vec2;

#[derive(Default, PartialEq)]
pub struct Player {
    size: Vec2<f32>,
    speed: f32
}

impl Player {
    pub fn new() -> Player {
        Player {
            size: Vec2::from_values(64., 64.),
            speed: 5.
        }
    }

    pub fn size(&self) -> Vec2<f32> { self.size }

    pub fn speed(&self) -> f32 { self.speed }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}