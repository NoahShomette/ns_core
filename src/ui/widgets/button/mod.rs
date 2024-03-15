use bevy::{app::Plugin, ecs::component::Component};

use self::basic_button::BasicButtonPlugin;

pub use self::basic_button::{basic_button, BasicButton, BasicButtonStyle};

mod basic_button;

pub struct ButtonCorePlugin;

impl Plugin for ButtonCorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(BasicButtonPlugin);
    }
}

/// Marks a button as selected
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct SelectedButton;

/// Marks a button as non interactable
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DisabledButton;
