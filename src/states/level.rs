use std::{
    time::{Duration, Instant},
};

use amethyst::{
    prelude::*,
    ecs::{World, Entity},
};

use crate::{
    resources::{
        UiHandles,
        MapConfig, MapHandle,
    }
};

use log;

pub struct LevelState {
    start_time: Instant,
    ui_root: Option<Entity>,

    map_config: MapConfig,
    map_handle: MapHandle,
}

impl LevelState {
    pub fn new(map_config: MapConfig, map_handle: MapHandle) -> Self {
        Self {
            start_time: Instant::now(),
            ui_root: None,
            map_config,
            map_handle
        }
    }

    fn build_ui(&mut self, world: &mut World) {
        log::info!("LevelState::build_ui");
        if self.ui_root.is_none() {
            let ui = {
                let handles = world.read_resource::<UiHandles>();
                handles.level.clone()
            };

            self.ui_root = Some(world.create_entity().with(ui).build());
        }
    }

    fn dispose_ui(&mut self, world: &mut World) {
        log::info!("LevelState::dispose_ui");
        if let Some(ui) = self.ui_root {
            let _ = world.delete_entity(ui);
            self.ui_root = None;
        }
        self.ui_root = None;
    }
}

impl SimpleState for LevelState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("LevelState::on_start");
        log::info!("LevelState::on_start - map is: {:#}", &self.map_config.display);
        self.build_ui(data.world);

        // let maps_storage: Fetch<AssetStorage<Map>> = data.world.read_resource::<AssetStorage<Map>>();
        // let map = maps_storage.get(&map_handle);
        // if let Some(m) = map {
        //     log::info!("SOME!");
        // } else {
        //     log::info!("SOME!");
        // }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("LevelState::on_stop");
        self.dispose_ui(data.world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.start_time.elapsed() < Duration::from_secs(2) {
            Trans::None
        } else {
            Trans::Pop
        }
    }
}