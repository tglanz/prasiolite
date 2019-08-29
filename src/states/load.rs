use std::{
    time::{Duration, Instant},
};

use amethyst::{
    prelude::{SimpleState, SimpleTrans, GameData, StateData, Trans},
    assets::{Completion, ProgressCounter},
    ui::{UiCreator},
    ecs::{World, Entity},
};

use crate::{
    resources,
    states::{MainMenuState}
};

use log;

pub struct LoadState {
    ui: Option<Entity>,
    start_time: Instant,
    progress_counter: ProgressCounter
}

impl LoadState {
    pub fn new() -> Self {
        Self {
            ui: None,
            start_time: Instant::now(),
            progress_counter: ProgressCounter::new(),
        }
    }

    pub fn dispose(&mut self, world: &mut World) {
        if let Some(ui) = self.ui {
            let _ = world.delete_entity(ui);
            self.ui = None;
        }
    }
}

impl SimpleState for LoadState {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("LoadState::on_start");

        let world = data.world;

        self.ui = Some(world.exec(|mut creator: UiCreator|
            creator.create("ui/load.ron", &mut self.progress_counter)));

        resources::load_resources(world, &mut self.progress_counter);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        match self.progress_counter.complete() {
            Completion::Loading => Trans::None,
            Completion::Failed => Trans::Quit,
            Completion::Complete => {
                if self.start_time.elapsed() < Duration::from_secs(2) {
                    Trans::None
                } else {
                    self.dispose(data.world);
                    Trans::Switch(Box::new(MainMenuState))
                }
            },
        }
    }
}