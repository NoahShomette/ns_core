//! # Example
//! ```rust
//!
//! // In the app build system
//!     app.setup_action::<CoreActions>(true);
//!
//! // Spawning your controls
//! pub fn setup_client_controller(mut commands: Commands) {
//!     commands.spawn((
//!         ClientController,
//!         InputManagerBundle {
//!             action_state: Default::default(),
//!             input_map: CoreActions::default_input_codes(),
//!         },
//!     ));
//! }
//! ```
//!
//! # Example of setting up your own actions for use
//! ```rust
//! #[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
//! pub enum CoreActions {
//!    ToggleDevMode,
//! }
//!
//! impl InputDefaultsTrait for CoreActions {
//!    type Action = CoreActions;
//!
//!    fn default_input_codes() -> InputMap<Self::Action> {
//!        let mut input_map = InputMap::default();
//!        input_map.insert_chord(
//!            CoreActions::ToggleDevMode,
//!            [KeyCode::AltRight, KeyCode::Backquote],
//!        );
//!        input_map
//!    }
//!}
//!
//! ```

use bevy::{
    app::{App, Update},
    ecs::{
        component::Component,
        event::{Event, EventReader},
        system::Commands,
    },
    reflect::{Reflect, TypePath},
};
use leafwing_input_manager::{
    input_map::InputMap,
    plugin::{InputManagerPlugin, ToggleActions},
    Actionlike,
};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A marker component for the clients controller entity
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct ClientController;

/// An event that will enable the given action
#[derive(Event, Default)]
pub struct EnableActionsEvent<A: Actionlike> {
    action_to_enable: PhantomData<A>,
}

/// A system that enables the given action when an [`EnableActionsEvent<Action>`] is received
fn enable_action<A: Actionlike>(
    mut event_reader: EventReader<EnableActionsEvent<A>>,
    mut commands: Commands,
) {
    if event_reader.read().next().is_some() {
        commands.insert_resource(ToggleActions::<A>::ENABLED)
    }
}

/// An event that will disable the given action
#[derive(Event, Default)]
pub struct DisableActionsEvent<A: Actionlike> {
    action_to_disable: PhantomData<A>,
}

fn disable_action<A: Actionlike>(
    mut event_reader: EventReader<DisableActionsEvent<A>>,
    mut commands: Commands,
) {
    if event_reader.read().next().is_some() {
        commands.insert_resource(ToggleActions::<A>::DISABLED)
    }
}

/// A trait to setup the specified action type.
///
/// Adds systems and events for managing that action.
pub trait ActionBusyworkTrait {
    fn setup_action<A: Actionlike + TypePath>(&mut self, enabled: bool);
}

impl ActionBusyworkTrait for App {
    fn setup_action<A: Actionlike + TypePath>(&mut self, enabled: bool) {
        if !enabled {
            self.insert_resource(ToggleActions::<A>::DISABLED);
        }
        self.add_event::<EnableActionsEvent<A>>()
            .add_event::<DisableActionsEvent<A>>();
        self.add_systems(Update, (disable_action::<A>, enable_action::<A>));

        self.add_plugins(InputManagerPlugin::<A>::default());
    }
}

/// A helper trait used to help when spawning Input Maps
pub trait InputDefaultsTrait {
    type Action: Actionlike + TypePath;
    fn default_input_codes() -> InputMap<Self::Action>;
}
