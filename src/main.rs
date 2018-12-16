extern crate amethyst;

use amethyst::prelude::*;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};
use amethyst::{
    ui::{DrawUi, UiBundle},
};

mod pong;
use crate::pong::Pong;

mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    use amethyst::utils::application_root_dir;
    use amethyst::input::InputBundle;

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());
    let input_handle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 0.7], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new())
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(input_handle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<String, String>::new())?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);
    
    let mut game = Application::new("./", Pong, game_data)?;

    game.run();

    Ok(())
}