//! An experimental state management system that is intended to automatically handle meta states.

use bevy::{app::Plugin, ecs::component::Component, prelude::States};

pub struct UnityScenePlugin<State: States> {
    state: State,
}

impl<State: States> Plugin for UnityScenePlugin<State> {
    fn build(&self, app: &mut bevy::prelude::App) {
        //app.add_systems(OnExit(State::), systems);
    }
}

#[derive(Component)]
pub struct ManagedByScene;

fn remove_scene_entities() {}
