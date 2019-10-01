use amethyst::{
    core::{WithNamed, Transform},
    ecs::{Builder, Component, HashMapStorage, World, WorldExt, Entity},
    renderer::{SpriteRender, Transparent}
};

use crate::load_sprite_sheet;

#[derive(Component, Default, PartialEq)]
#[storage(HashMapStorage)]
pub struct Player {
    speed: f32
}

impl Player {
    fn new() -> Player {
        Player {
            speed: 5.
        }
    }

    pub fn speed(&self) -> f32 { self.speed }

    pub fn init(world: &mut World) -> Entity {
        let sprite_sheet = load_sprite_sheet(world, "player_sheet.png");

        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., -3.);

        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 1
        };

        world.create_entity()
            .with(transform)
            .with(sprite)
            .with(Player::new())
            .with(Transparent)
            .named("player")
            .build()
    }

}