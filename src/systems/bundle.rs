use amethyst::{
    error::Error,
    core::bundle::SystemBundle,
    ecs::prelude::DispatcherBuilder
};

pub struct Bundle;

impl<'a, 'b> SystemBundle<'a, 'b> for Bundle {
    fn build(self, _builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        // builder.add(super::map_spawn::MapSpawnSystem, "prasiolite::map_spawn_system", &[]);
        Ok(())
    }
}