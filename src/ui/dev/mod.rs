use bevy::{
    app::Plugin,
    ecs::schedule::{apply_deferred, IntoSystemConfigs},
};

use crate::controls::setup_client_controller;

use self::dev_modal::{setup_dev_modal, DevModal};

use super::{marker_component, UiOneShotSystemAppExtension};

pub mod actions;
mod dev_modal;

pub struct DevModePlugin;

impl Plugin for DevModePlugin {
    #[allow(unused)]
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_one_shot_system(marker_component::<DevModal>(), setup_dev_modal);

        use self::actions::{dev_actions, DevActions};
        use crate::controls::{ActionBusyworkTrait, ClientController, InputDefaultsTrait};
        use bevy::{
            app::{Startup, Update},
            ecs::{
                entity::Entity,
                query::With,
                system::{Commands, Query},
            },
        };
        use leafwing_input_manager::prelude::ToggleActions;
        use leafwing_input_manager::InputManagerBundle;

        fn setup_dev_mode(
            controller_query: Query<Entity, With<ClientController>>,
            mut commands: Commands,
        ) {
            let entity = controller_query.single();

            commands.entity(entity).insert(InputManagerBundle {
                action_state: Default::default(),
                input_map: DevActions::default_input_codes(),
            });
        }
        app.setup_action::<DevActions>(true);
        app.add_systems(
            Startup,
            (apply_deferred, setup_dev_mode)
                .chain()
                .after(setup_client_controller),
        );
        app.add_systems(Update, dev_actions);
    }
}
