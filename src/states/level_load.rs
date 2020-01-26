use std::{
    time::{Duration, Instant},
};

use amethyst::{
    prelude::*,
    ecs::{World, Entity},
    assets::{AssetStorage, Completion, ProgressCounter, Loader},
};

use crate::{
    states::LevelState,

    resources::{
        UiHandles,
        MapConfig,
        TmxFormat,
        Map,
        MapHandle,
    }
};

use log;

pub struct LevelLoadState {
    start_time: Instant,
    map_config: MapConfig,
    progress_counter: ProgressCounter,
    ui_root: Option<Entity>,
    map_handle: Option<MapHandle>,
}

impl LevelLoadState {
    pub fn new(map_config: MapConfig) -> Self {
        Self {
            start_time: Instant::now(),
            map_config,
            progress_counter: ProgressCounter::new(),
            ui_root: None,
            map_handle: None,
        }
    }

    fn build_ui(&mut self, world: &mut World) {
        log::info!("LevelLoadState::build_ui");
        if self.ui_root.is_none() {
            let ui = {
                let handles = world.read_resource::<UiHandles>();
                handles.level_load.clone()
            };

            self.ui_root = Some(world.create_entity().with(ui).build());
        }
    }

    fn dispose_ui(&mut self, world: &mut World) {
        log::info!("LevelLoadState::dispose_ui");
        if let Some(ui) = self.ui_root {
            let _ = world.delete_entity(ui);
            self.ui_root = None;
        }
    }
}

impl SimpleState for LevelLoadState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("LevelLoadState::on_start");
        self.build_ui(data.world);

        let path = ["maps/", &self.map_config.file_name, ".tmx"].join("");
        let loader = data.world.read_resource::<Loader>();

        self.map_handle = Some(loader.load(
            path,
            TmxFormat::default(),
            &mut self.progress_counter,
            &data.world.read_resource::<AssetStorage<Map>>()
        ));

        // let map_handle = {
        //     data.world.read_resource::<MapsHandles>().map1.clone()
        // };

        // let maps_storage: Fetch<AssetStorage<Map>> = data.world.read_resource::<AssetStorage<Map>>();
        // let map = maps_storage.get(&map_handle);
        // if let Some(m) = map {
        //     log::info!("SOME!");
        // } else {
        //     log::info!("None!");
        // }
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("LevelLoadState::on_stop");
        self.dispose_ui(data.world);
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress_counter.complete() {
            Completion::Loading => {
                log::info!("LevelLoadState::update - Still Loading");
                Trans::None
            },
            Completion::Failed => Trans::Quit,
            Completion::Complete => {
                if self.start_time.elapsed() < Duration::from_secs(2) {
                    Trans::None
                } else {
                    Trans::Switch(Box::from(LevelState::new(
                        self.map_config.clone(),
                        std::mem::replace(&mut self.map_handle, None).unwrap()
                    )))
                }
            },
        }       
    }
}