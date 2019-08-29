use amethyst::{
    prelude::{SimpleState, SimpleTrans, GameData, StateData, Trans},
};

use crate::{
    states::{LoadState},
};

use log;

pub struct OnBoardState;

impl SimpleState for OnBoardState {

    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        log::info!("OnBoardState::on_start");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Switch(Box::new(LoadState::new()))
    }
}