use amethyst::{
    error::{Error},
    ecs::{World, VecStorage},
    assets::{Asset, ProgressCounter, Handle, ProcessingState, Format, Loader, AssetStorage},
};

use super::Loadable;

pub struct Map {
    pub tiled_map: tiled::Map,
}

type MapHandle = Handle<Map>;

pub struct MapsHandles {
    pub map1: MapHandle,
}

impl From<tiled::Map> for Map {
    fn from(tiled_map: tiled::Map) -> Self {
        Map {
            tiled_map,
        }
    }
}

impl Asset for Map {
    const NAME: &'static str = "prasiolite::resources::Map";
    type Data = Self;
    type HandleStorage = VecStorage<MapHandle>;
}

impl From<Map> for Result<ProcessingState<Map>, Error> {
    fn from(map: Map) -> Result<ProcessingState<Map>, Error> {
        let processing_state = ProcessingState::Loaded(map);
        Ok(processing_state)
    }
}

/// Format for loading from `.tmx` files.
#[derive(Clone, Copy, Debug, Default)]
pub struct TmxFormat;

impl Format<Map> for TmxFormat {
    fn name(&self) -> &'static str {
        "TmxFormat"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<Map, Error> {
        match tiled::parse(&bytes[..]) {
            Ok(tiled_map) => Ok(Map::from(tiled_map)),
            _ => Err(Error::from_string("error, match to se what is the error exactly")),
        }
    }
}

pub fn load_map(world: &mut World, progress_counter: &mut ProgressCounter, name: &str) -> Handle<Map> {
    let path = ["maps/", name, ".tmx"].join("");
    let loader = &world.read_resource::<Loader>();

    loader.load(
        path,
        TmxFormat::default(),
        progress_counter,
        &world.read_resource::<AssetStorage<Map>>(),
    )
}

impl Loadable for MapsHandles {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        MapsHandles {
            map1: load_map(world, progress_counter, "map1")
        }
    }
}