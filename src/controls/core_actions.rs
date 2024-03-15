use bevy::{input::keyboard::KeyCode, reflect::Reflect};
use leafwing_input_manager::{input_map::InputMap, Actionlike};

use super::InputDefaultsTrait;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CoreActions {
    ToggleDevMode,
}

impl InputDefaultsTrait for CoreActions {
    type Action = CoreActions;

    fn default_input_codes() -> InputMap<Self::Action> {
        let mut input_map = InputMap::default();
        input_map.insert_chord(
            [KeyCode::AltRight, KeyCode::Grave],
            CoreActions::ToggleDevMode,
        );
        input_map
    }
}
