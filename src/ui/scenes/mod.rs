//! Scenes are essentially distinct Ui modes.
//!
//! They represent a logical point in the app delineated by the state fed into it where you want the given ui to be displayed.
//! Internally the plugin will manage spawning and despawning the scene and all the components marked with the given marker componet
//! from the app when the scene is entered or left

use bevy::{
    app::App, ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, IntoSystem, Query, Res},
    }, hierarchy::DespawnRecursiveExt, prelude::{OnEnter, OnExit, States}, reflect::TypePath
};

use crate::one_shot_system::{MarkerComponent, OneShotSystemIds};

pub trait ScenesAppExtension {
    /// Adds a new scene that will run the setup system every time the given state is entered and a cleanup system every time it leaves.
    fn add_scene<Marker: Component + TypePath, M>(
        &mut self,
        marker_component: MarkerComponent<Marker>,
        setup_system: impl IntoSystem<(), (), M> + 'static,
        state: impl States,
    );
}

impl ScenesAppExtension for App {
    fn add_scene<Marker: Component + TypePath, M>(
        &mut self,
        _: MarkerComponent<Marker>,
        setup_system: impl IntoSystem<(), (), M> + 'static,
        states: impl States,
    ) {
        let system_id = self.world_mut().register_system(setup_system);
        let mut resource = self.world_mut().resource_mut::<OneShotSystemIds>();
        resource.map.insert(Marker::type_path(), system_id);
        self.add_systems(OnEnter(states.clone()), setup_scene::<Marker>);
        self.add_systems(OnExit(states), cleanup_scene::<Marker>);
    }
}

fn setup_scene<SceneRootMarker: Component + TypePath>(
    mut commands: Commands,
    resource: Res<OneShotSystemIds>,
) {
    let system_id = resource.map.get(SceneRootMarker::type_path()).unwrap();
    commands.run_system(*system_id);
}

fn cleanup_scene<SceneRootMarker: Component>(
    query: Query<Entity, With<SceneRootMarker>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
