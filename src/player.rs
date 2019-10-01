use amethyst::{
    ecs::{Component, HashMapStorage}
};

#[derive(Component, Default, PartialEq)]
#[storage(HashMapStorage)]
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