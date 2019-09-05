use amethyst::{
    prelude::*,
    ecs::{Entity},
    ui::{UiEventType, UiFinder, UiEvent},
};

use crate::{
    resources::{UiHandles},
};

use log;

#[derive(Default)]
pub struct MainMenuState {
    ui_root: Option<Entity>,
    exit_button: Option<Entity>,
}

impl MainMenuState {
    fn pick_ui_if_needed(&mut self, world: &mut World) {
        world.exec(|ui_finder: UiFinder<'_>| {
            self.exit_button = ui_finder.find("exit-button");
        });
    }
}

impl SimpleState for MainMenuState {

    fn on_start(&mut self, data: StateData<GameData<'_, '_>>) {
        log::info!("MainMenuState::on_start");

        let ui = {
            let handles = data.world.read_resource::<UiHandles>();
            handles.main_menu.clone()
        };

        self.ui_root = Some(data.world.create_entity().with(ui).build());
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