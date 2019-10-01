use amethyst::{
    prelude::*,
    core::{Named, Parent, Transform},
    ecs::{Entity, ReadStorage, Join},
    input::{is_close_requested, is_key_down},
    renderer::Camera,
    window::ScreenDimensions,
    winit
};

use crate::player::Player;

pub struct Winter;

impl SimpleState for Winter {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Named>();


        let player = Player::init(world);
        let _camera = init_camera(world, player);
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let StateData { world, .. } = data;

        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            }
            else if is_key_down(&event, winit::VirtualKeyCode::Space) {
                world.exec(|(named, transforms): (ReadStorage<Named>, ReadStorage<Transform>)| {
                    for (name, transform) in (&named, &transforms).join() {
                        println!("{} => {:?}", name.name, transform.translation());
                    }
                });

                Trans::None
            }
            else { Trans::None }
        }
        else { Trans::None }
    }
}

fn init_camera(world: &mut World, to_follow: Entity) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transform = Transform::default();
    transform.set_translation_z(5.);

    world.create_entity()
        .with(transform)
        .with(Parent { entity: to_follow })
        .with(Camera::standard_2d(width, height))
        .named("camera")
        .build()
}