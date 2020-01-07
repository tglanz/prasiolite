use amethyst::{
    error::Error,
    core::bundle::SystemBundle,
    assets::Processor,
    ecs::prelude::DispatcherBuilder
};

use super::{
    maps_handles::Map
};

pub struct Bundle;

impl<'a, 'b> SystemBundle<'a, 'b> for Bundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(Processor::<Map>::new(), "prasiolite::MapProcessor", &[]);
        Ok(())
    }
}
