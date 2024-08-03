use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin},
    ecs::{
        component::Component,
        system::{IntoSystem, Resource, SystemId},
    },
    reflect::TypePath,
    utils::HashMap,
};

/// Plugin for the one shot system extensions
pub struct OneShotSystemPlugin;

impl Plugin for OneShotSystemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<OneShotSystemIds>();
    }
}

#[derive(Resource, Default)]
pub struct OneShotSystemIds {
    pub map: HashMap<&'static str, SystemId>,
}

pub trait OneShotSystemAppExtension {
    /// Registers the given system using the given marker component [`TypePath`].
    fn register_one_shot_system<Marker: Component + TypePath, M>(
        &mut self,
        _: MarkerComponent<Marker>,

        system: impl IntoSystem<(), (), M> + 'static,
    );
}

impl OneShotSystemAppExtension for App {
    fn register_one_shot_system<Marker: Component + TypePath, M>(
        &mut self,
        _: MarkerComponent<Marker>,

        system: impl IntoSystem<(), (), M> + 'static,
    ) {
        let system_id = self.world_mut().register_system(system);
        let mut resource = self.world_mut().resource_mut::<OneShotSystemIds>();
        resource.map.insert(Marker::type_path(), system_id);
    }
}

/// Helper function to make creating [`MarkerComponent`] function type helpers easier
///
///
pub fn marker_component<Marker: Component + TypePath>() -> MarkerComponent<Marker> {
    MarkerComponent::<Marker> { pd: PhantomData }
}

pub struct MarkerComponent<C: Component + TypePath> {
    pd: PhantomData<C>,
}
