use amethyst::{
    ecs::{Component, DenseVecStorage}
};

#[derive(Default, PartialEq)]
pub struct Player {
    speed: f32
}

impl Player {
    pub fn new() -> Player {
        Player {
            speed: 5.
        }
    }

    pub fn speed(&self) -> f32 { self.speed }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}