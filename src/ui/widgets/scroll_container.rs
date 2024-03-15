use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    prelude::default,
    ui::{node_bundles::NodeBundle, Style},
};
use bevy_eventlistener::event_listener::On;

use super::scroll::{scroll, ScrollEvent};

/// Settings used to construct a scroll container
pub struct ScrollContainerSettings {
    pub style: Style,
}

/// Construct and spawn a new tabbed content container. Returns the entity for the entire tabbed content and the entity for each tab.
pub fn scroll_container<T>(
    menu_type: Option<T>,
    scroll_settings: ScrollContainerSettings,
    commands: &mut Commands,
) -> Entity
where
    T: Component,
{
    // Root level node, spanning the whole screen and applying a 50% opacity
    let root = commands
        .spawn((
            NodeBundle {
                style: scroll_settings.style,
                ..default()
            },
            On::<ScrollEvent>::run(scroll),
        ))
        .id();

    if let Some(menu_type) = menu_type {
        commands.entity(root).insert(menu_type);
    }

    root
}
