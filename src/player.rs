use amethyst::{
    core::{Transform},
    ecs::{Join, Component, NullStorage, Read, ReadStorage, WriteStorage, System},
    input::{StringBindings, InputHandler}
};

use crate::systems::collision::CollisionRect;
use crate::math::{Vec2, Rect};

#[derive(Default)]
pub struct Player {
    collision_rect: CollisionRect
}

impl Player {
    pub fn new(pos: Vec2<f32>) -> Player {
        Player {
            collision_rect: Rect::from_slice([pos.x, pos.y, 64., 64.])
        }
    }

    pub fn set_pos(&mut self, pos: Vec2<f32>) {
        self.collision_rect.set_pos(pos);
    }

    pub fn collision_rect(&self) -> &CollisionRect {
        &self.collision_rect
    }

    pub fn translate(&mut self, by: Vec2<f32>) {
        self.collision_rect.translate(by);
    }
}

impl Component for Player {
    type Storage = NullStorage<Self>;
}

// System, das dafür sorgt, dass die Kamera immer den Spieler verfolgt, da die
// Welt zu groß ist, sie im Ganzen zu zeigen
struct NormalMovementSystem;

impl<'s> System<'s> for NormalMovementSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut players, mut transforms, input): Self::SystemData) {
        let x_dir = input.axis_value("player_x").unwrap();
        let y_dir = input.axis_value("player_y").unwrap();
        let dir: Vec2<f32> = Vec2::from_values(x_dir, y_dir);
        let dir = dir / dir.len();

        for (player, transform) in (&mut players, &mut transforms).join() {
            transform.prepend_translation_x(dir.x);
            transform.prepend_translation_y(dir.y);

            player.set_pos(Vec2::from_values(transform.translation().x, transform.translation().y));
        }
    }
}