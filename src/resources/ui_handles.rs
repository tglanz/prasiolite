use amethyst::{
    ecs::{World},
    assets::{ProgressCounter, Handle},
    ui::{UiLoader, UiPrefab},
};

use super::Loadable;

pub struct UiHandles {
    pub main_menu: Handle<UiPrefab>,
    pub level: Handle<UiPrefab>,
    pub level_load: Handle<UiPrefab>,
}

pub fn load_ui_prefab(world: &mut World, progress_counter: &mut ProgressCounter, name: &str) -> Handle<UiPrefab> {
    world.exec(|loader: UiLoader<'_>| {
        let path = format!("ui/{}.ron", name);
        loader.load(path, &mut *progress_counter)
    })
}

impl Loadable for UiHandles {
    fn load(world: &mut World, progress_counter: &mut ProgressCounter) -> Self {
        UiHandles {
            main_menu: load_ui_prefab(world, &mut *progress_counter, "main-menu"),
            level: load_ui_prefab(world, &mut *progress_counter, "level"),
            level_load: load_ui_prefab(world, &mut *progress_counter, "level_load"),
        }
    }
}