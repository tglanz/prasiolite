use amethyst::{
    prelude::{SimpleState, SimpleTrans, GameData, StateData, Trans, Builder},
};

use crate::{
    resources::{UiHandles},
};

use log;

pub struct MainMenuState;

impl SimpleState for MainMenuState {

    fn on_start(&mut self, data: StateData<GameData<'_, '_>>) {
        log::info!("MainMenuState::on_start");

        let ui = {
            let handles = data.world.read_resource::<UiHandles>();
            handles.main_menu.clone()
        };

        data.world.create_entity().with(ui).build();
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }
}