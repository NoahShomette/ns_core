use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Query, Res},
    },
    input::keyboard::KeyCode,
    reflect::{Reflect, TypePath},
};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap, Actionlike};

use crate::{controls::InputDefaultsTrait, ui::UiSystemIdResource};

use super::dev_modal::DevModal;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum DevActions {
    OpenDevModal,
}

impl InputDefaultsTrait for DevActions {
    type Action = DevActions;

    fn default_input_codes() -> InputMap<Self::Action> {
        InputMap::new([(DevActions::OpenDevModal, KeyCode::Backquote)])
    }
}

pub fn dev_actions(
    actions_query: Query<(Entity, &ActionState<DevActions>)>,
    dev_modal_query: Query<&DevModal>,
    mut commands: Commands,
    ui_systems: Res<UiSystemIdResource>,
) {
    let Ok((_entity, actions)) = actions_query.get_single() else {
        return;
    };

    if actions.just_pressed(&DevActions::OpenDevModal) {
        if !dev_modal_query.is_empty() {
            return;
        }

        commands.run_system(
            ui_systems
                .map
                .get(DevModal::type_path())
                .expect("Dev Modal setup system not registered")
                .clone(),
        );
    }
}
