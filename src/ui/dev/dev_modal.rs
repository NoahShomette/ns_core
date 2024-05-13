use bevy::{
    ecs::{
        component::Component,
        event::EventWriter,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    prelude::default,
    reflect::TypePath,
    ui::{node_bundles::NodeBundle, AlignItems, JustifyContent, PositionType, Style, Val},
};
use bevy_eventlistener::{callbacks::ListenerMut, event_listener::On};
use bevy_mod_picking::events::{Click, Pointer};

use crate::ui::{
    colors::CurrentColors,
    widgets::{
        button::{basic_button, BasicButtonStyle},
        modal::{modal_panel, ModalStyle},
    },
};

#[derive(Component, TypePath)]
pub struct DevModal;

pub fn setup_dev_modal(mut commands: Commands, colors: Res<CurrentColors>) {
    let modal = modal_panel(
        DevModal,
        ModalStyle {
            can_close: true,
            close_button_bundle: None::<()>,
            modal_size: Some((Val::Percent(75.0), Val::Percent(75.0))),
            outline: true,
        },
        &colors,
        &mut commands,
    );
    let button_container = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        })
        .id();

    let button_style = BasicButtonStyle {
        bundle: None::<()>,
        text: "Refresh Access Token".to_string(),
        font_size: 40.0,
    };

    commands.entity(button_container).push_children(&[]);
    commands.entity(modal).push_children(&[button_container]);
}
