use std::marker::PhantomData;

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        component::Component,
        query::With,
        schedule::{common_conditions::resource_exists, IntoSystemConfigs},
        system::{IntoSystem, Local, Query, ResMut, Resource, SystemId},
    },
    math::Vec2,
    reflect::TypePath,
    ui::UiScale,
    utils::HashMap,
    window::{PrimaryWindow, Window},
};
use bevy_simple_text_input::TextInputPlugin;

use self::{colors::GameColorsPlugin, widgets::WidgetsPlugin};

pub mod colors;
pub mod dev;
pub mod scenes;
pub mod widgets;

pub struct NsCoreUiPlugin;

impl Plugin for NsCoreUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<UiSystemIdResource>();
        app.add_plugins(TextInputPlugin);
        app.add_plugins((WidgetsPlugin, GameColorsPlugin));

        #[cfg(debug_assertions)]
        {
            use self::dev::DevModePlugin;
            app.add_plugins(DevModePlugin);
        }
        app.add_systems(Update, scale.run_if(resource_exists::<ScaleUi>));
    }
}

pub const UI_SCREEN_LAYER: i32 = 1;
pub const UI_MODAL_LAYER: i32 = 100;

#[derive(Resource, Default)]
pub struct ScaleUi;

#[derive(Resource, Default)]
pub struct UiSystemIdResource {
    pub map: HashMap<&'static str, SystemId>,
}

pub trait UiOneShotSystemAppExtension {
    /// Registers the given system using the given marker component [`TypePath`].
    fn register_one_shot_system<Marker: Component + TypePath, M>(
        &mut self,
        _: MarkerComponent<Marker>,

        system: impl IntoSystem<(), (), M> + 'static,
    );
}

impl UiOneShotSystemAppExtension for App {
    fn register_one_shot_system<Marker: Component + TypePath, M>(
        &mut self,
        _: MarkerComponent<Marker>,

        system: impl IntoSystem<(), (), M> + 'static,
    ) {
        let system_id = self.world.register_system(system);
        let mut resource = self.world.resource_mut::<UiSystemIdResource>();
        resource.map.insert(Marker::type_path(), system_id);
    }
}

/// Scales ui to match the screen
pub fn scale(
    mut cached_size: Local<Vec2>,
    mut ui_scale: ResMut<UiScale>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(primary) = windows.iter().next() else {
        return;
    };
    let ww = primary.width();
    let wh = primary.height();
    if cached_size.x == ww && cached_size.y == wh {
        return;
    }
    cached_size.x = ww;
    cached_size.y = wh;

    let scale_h = ww / 1920.0;
    let scale_w = wh / 1080.0;
    ui_scale.0 = scale_h.min(scale_w);
}

/// Helper function to make creating [`MarkerComponent`] function type helpers easier
pub fn marker_component<Marker: Component + TypePath>() -> MarkerComponent<Marker> {
    MarkerComponent::<Marker> { pd: PhantomData }
}

pub struct MarkerComponent<C: Component + TypePath> {
    pd: PhantomData<C>,
}
