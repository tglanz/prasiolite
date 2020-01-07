use amethyst::{
    ecs::*
};

use crate::{
    resources::maps_handles::{MapsHandles, Map, MapHandle}
};

pub struct MapSpawnSystem;

impl<'s> System<'s> for MapSpawnSystem {
    fn run(&mut self, data: Self::SystemData) {

        

        println!("running system")
    }
}