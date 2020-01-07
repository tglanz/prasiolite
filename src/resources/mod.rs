pub mod ui_handles;
pub mod maps_handles;
pub mod bundle;

use amethyst::{
    ecs::{World},
    assets::{ProgressCounter},
};

pub use ui_handles::UiHandles;
pub use maps_handles::{MapsHandles, Map};
pub use bundle::Bundle;

trait Loadable {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self;
}

pub fn load_resources(world: &mut World, progress_counter: &mut ProgressCounter) {
    let ui_handles = UiHandles::load(world, progress_counter);
    world.add_resource(ui_handles);

    let maps_handles = MapsHandles::load(world, progress_counter);
    world.add_resource(maps_handles);
}