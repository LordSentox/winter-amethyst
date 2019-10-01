use amethyst::{
    prelude::*,
    assets::{AssetStorage, Handle, Loader},
    core::{Named, Parent, Transform},
    ecs::{Entity, ReadStorage, Join},
    input::{is_close_requested, is_key_down},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, Transparent},
    window::ScreenDimensions,
    winit
};

use crate::player::Player;

pub struct Winter;

impl SimpleState for Winter {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Named>();

        let player_sprite_sheet_handle = load_sprite_sheet(world, "player_sheet.png");

        let player = init_player(world, &player_sprite_sheet_handle);
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

fn init_player(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) -> Entity {
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

fn load_sprite_sheet(world: &mut World, image_path: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(image_path, ImageFormat::default(), (), &texture_storage)
    };

    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    let mut ron_path = String::from(image_path.split_at(image_path.rfind('.').expect("Image has no file type")).0);
    ron_path.push_str(".ron");

    loader.load(ron_path, SpriteSheetFormat(texture_handle), (), &sprite_sheet_store)
}