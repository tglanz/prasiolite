mod ui_handles;
mod maps_handles;
mod bundle;
mod maps_config;

use amethyst::{
    ecs::World,
    assets::ProgressCounter,
    config::Config,
};

pub use ui_handles::UiHandles;
pub use bundle::Bundle;
pub use maps_config::{MapsConfig, MapConfig};
pub use maps_handles::{MapHandle, Map, TmxFormat};

trait Loadable {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self;
}

pub fn load_resources(world: &mut World, progress_counter: &mut ProgressCounter) {
    let ui_handles = UiHandles::load(world, progress_counter);
    world.add_resource(ui_handles);

    let maps_config = MapsConfig::load("resources/configs/maps.ron");
    world.add_resource(maps_config);
}