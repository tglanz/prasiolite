pub mod ui_handles;

use amethyst::{
    ecs::{World},
    assets::{ProgressCounter},
};

pub use ui_handles::UiHandles;

trait Loadable {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self;
}

pub fn load_resources(world: &mut World, progress_counter: &mut ProgressCounter) {
    let ui_handles = UiHandles::load(world, progress_counter);
    world.add_resource(ui_handles);
}