use amethyst::{
    prelude::*,
    core::{
        transform::TransformBundle,
    },
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{UiBundle, RenderUi},
    input::{StringBindings, InputBundle},
    utils::application_root_dir,
};

mod resources;
mod states;
mod systems;

pub fn run() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");

    let display_config_path = resources.join("configs/display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(resources::Bundle)?
        .with_bundle(systems::Bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?;
        

    let mut game = Application::new(resources, states::OnBoardState, game_data)?;
    game.run();

    Ok(())
}