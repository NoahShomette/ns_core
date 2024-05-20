use bevy::{
    app::{Plugin, Update},
    ecs::{
        query::With,
        schedule::{common_conditions::resource_exists, IntoSystemConfigs},
        system::{Local, Query, ResMut, Resource},
    },
    math::Vec2,
    ui::UiScale,
    window::{PrimaryWindow, Window},
};
use bevy_simple_text_input::TextInputPlugin;

use self::{
    colors::{CurrentColors, GameColorsPlugin},
    widgets::WidgetsPlugin,
};

pub mod colors;
pub mod scenes;
pub mod widgets;

pub struct NsCoreUiPlugin {
    pub custom_colors: Option<CurrentColors>,
}

impl Plugin for NsCoreUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(TextInputPlugin);
        app.add_plugins((
            WidgetsPlugin,
            GameColorsPlugin {
                custom_colors: self.custom_colors.clone(),
            },
        ));
        app.add_systems(Update, scale.run_if(resource_exists::<ScaleUi>));
    }
}

pub const UI_SCREEN_LAYER: i32 = 1;
pub const UI_MODAL_LAYER: i32 = 100;

#[derive(Resource, Default)]
pub struct ScaleUi;

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
