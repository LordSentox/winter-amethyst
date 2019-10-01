use amethyst::{
    prelude::*,
    assets::{AssetStorage, Handle, Loader},
    core::TransformBundle,
    input::{InputBundle, StringBindings},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        RenderingBundle,
        types::DefaultBackend,
        ImageFormat, SpriteSheet, SpriteSheetFormat, Texture
    },
    window::DisplayConfig,
    utils::application_root_dir
};

use amethyst::LogLevelFilter;

mod math;
mod map;
mod player;
mod systems;
mod winter;

use winter::Winter;

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("amethyst_assets", LogLevelFilter::Debug)
        .start();

    let app_root = application_root_dir()?;
    //let app_root = std::path::Path::new("/home/sentox/External/Code/winter-amethyst"); // FIXME: Just for debugging
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");

    println!("Assets root: {:?}", assets_dir);

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(config_dir.join("input.ron"))?;
    let mut display_config = DisplayConfig::default();
    display_config.title = "Winter".to_string();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
                                .with_plugin(RenderToWindow::from_config(display_config).with_clear([0., 0., 0., 1.]))
                                .with_plugin(RenderFlat2D::default());

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(rendering_bundle)?
        .with(systems::PlayerMovement, "player_movement", &[])
        .with(systems::Collision, "collision", &[]);

    let mut game = Application::build(assets_dir, Winter)?.build(game_data)?;
    game.run();

    Ok(())
}

pub fn load_sprite_sheet(world: &mut World, image_path: &str) -> Handle<SpriteSheet> {
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