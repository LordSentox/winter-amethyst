use amethyst::{
    core::Transform,
    input::{InputHandler, StringBindings},
    ecs::{Join, System, ReadStorage, WriteStorage, Read}
};

use crate::player::Player;
use crate::math::Vec2;

pub struct PlayerMovement;

impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut players, mut transforms, input): Self::SystemData) {
        let x_dir = input.axis_value("player_x").unwrap();
        let y_dir = input.axis_value("player_y").unwrap();

        // Create a normalised movement vector to work with
        let dir: Vec2<f32> = Vec2::from_values(x_dir, y_dir);
        let dir = dir / dir.len();

        for (player, transform) in (&players, &mut transforms).join() {
            transform.prepend_translation_x(dir.x * player.speed());
            transform.prepend_translation_y(dir.y * player.speed());
        }
    }
}