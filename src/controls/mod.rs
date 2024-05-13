use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        component::Component,
        event::{Event, EventReader},
        system::Commands,
    },
    reflect::TypePath,
};
use leafwing_input_manager::{
    input_map::InputMap,
    plugin::{InputManagerPlugin, ToggleActions},
    Actionlike, InputManagerBundle,
};

use self::core_actions::CoreActions;

pub mod core_actions;

/// Core actions plugin
pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.setup_action::<CoreActions>(true);
        app.add_systems(Startup, setup_client_controller);
    }
}

/// A marker component for the clients controller entity
#[derive(Component)]
pub struct ClientController;

pub fn setup_client_controller(mut commands: Commands) {
    commands.spawn((
        ClientController,
        InputManagerBundle {
            action_state: Default::default(),
            input_map: CoreActions::default_input_codes(),
        },
    ));
}

/// An event that will enable the given action
#[derive(Event)]
pub struct EnableActionsEvent<A: Actionlike> {
    action_to_enable: PhantomData<A>,
}

fn enable_actions<A: Actionlike>(
    mut event_reader: EventReader<EnableActionsEvent<A>>,
    mut commands: Commands,
) {
    if event_reader.read().next().is_some() {
        commands.insert_resource(ToggleActions::<A>::ENABLED)
    }
}

/// An event that will disable the given action
#[derive(Event)]
pub struct DisableActionsEvent<A: Actionlike> {
    action_to_disable: PhantomData<A>,
}

fn disable_actions<A: Actionlike>(
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
        self.add_systems(Update, (disable_actions::<A>, enable_actions::<A>));

        self.add_plugins(InputManagerPlugin::<A>::default());
    }
}

pub trait InputDefaultsTrait {
    type Action: Actionlike + TypePath;
    fn default_input_codes() -> InputMap<Self::Action>;
}
