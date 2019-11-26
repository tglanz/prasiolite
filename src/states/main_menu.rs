use amethyst::{
    prelude::*,
    ecs::{Entity},
    ui::{UiEventType, UiFinder, UiEvent},
};

use crate::{
    states::{LevelState},
    resources::{UiHandles},
};

use log;

#[derive(Default)]
pub struct MainMenuState {
    ui_root: Option<Entity>,
    exit_button: Option<Entity>,
    start_button: Option<Entity>,
}

impl MainMenuState {
    fn pick_ui_if_needed(&mut self, world: &mut World) {
        if self.ui_root.is_some() && self.exit_button.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.exit_button = ui_finder.find("exit-button");
                self.start_button = ui_finder.find("start-button");
            });
        }
    }

    fn build_ui(&mut self, world: &mut World) {
        log::info!("MainState::build_ui");
        if self.ui_root.is_none() {
            let ui = {
                let handles = world.read_resource::<UiHandles>();
                handles.main_menu.clone()
            };

            self.ui_root = Some(world.create_entity().with(ui).build());
        }
    }

    fn dispose_ui(&mut self, world: &mut World) {
        log::info!("MainMenuState::dispose_ui");
        if let Some(ui) = self.ui_root {
            let _ = world.delete_entity(ui);
            self.ui_root = None;
        }
        self.ui_root = None;
        self.exit_button = None;
        self.start_button = None;
    }
}

impl SimpleState for MainMenuState {

    fn on_start(&mut self, data: StateData<GameData<'_, '_>>) {
        log::info!("MainMenuState::on_start");
        self.build_ui(data.world)
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("MainMenuState::on_stop");
        self.dispose_ui(data.world);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("MainMenuState::on_resume");
        self.build_ui(data.world);
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        log::info!("MainMenuState::on_pause");
        self.dispose_ui(data.world);
    }
 
    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target
            }) => {
                if Some(target) == self.exit_button {
                    return Trans::Quit;
                }

                if Some(target) == self.start_button {
                    return Trans::Push(Box::from(LevelState::new()));
                }
            },
            _ => {}
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.pick_ui_if_needed(data.world);
        Trans::None
    }
}