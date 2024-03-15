use bevy::{
    app::{Plugin, Update},
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query},
        world::Ref,
    },
    hierarchy::{BuildChildren, Children, Parent},
    prelude::default,
    ui::{
        node_bundles::NodeBundle, widget::Button, AlignItems, Display, FlexDirection, FocusPolicy,
        Interaction, JustifyContent, PositionType, Style, UiRect, Val,
    },
};

use crate::ui::colors::CurrentColors;

use super::button::{basic_button, BasicButton, BasicButtonStyle, DisabledButton, SelectedButton};

pub struct TabbedContentPlugin;

impl Plugin for TabbedContentPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, tab_button_interaction);
    }
}

pub fn tab_button_interaction(
    tab_buttons: Query<
        (
            Entity,
            Ref<Interaction>,
            &TabContentEntity,
            &Parent,
            Option<&SelectedButton>,
        ),
        ((With<Button>, With<BasicButton>, Without<DisabledButton>),),
    >,
    children: Query<&Children>,
    mut tab_content: Query<(&mut Style, &TabContent), Without<TabContentEntity>>,
    mut commands: Commands,
) {
    for (interacted_entity, interaction, tab_content_entity, parent, _) in tab_buttons.iter() {
        if !interaction.is_changed() {
            continue;
        }

        if let Interaction::Pressed = *interaction {
            if let Ok(parent_children) = children.get(parent.get()) {
                for child in parent_children.iter() {
                    let Ok((tab_entity, _, other_tab_content_entity, _, option_selected)) =
                        tab_buttons.get(*child)
                    else {
                        continue;
                    };

                    if option_selected.is_some() && *child != interacted_entity {
                        commands.entity(tab_entity).remove::<SelectedButton>();
                    }

                    if let Ok((mut style, _)) = tab_content.get_mut(other_tab_content_entity.0) {
                        style.display = Display::None;
                    }
                }
            }

            if let Ok((mut style, _)) = tab_content.get_mut(tab_content_entity.0) {
                style.display = Display::DEFAULT;
            }
            commands.entity(interacted_entity).insert(SelectedButton);
        }
    }
}

#[derive(Component)]
pub struct TabContentEntity(Entity);

#[derive(Component)]
pub struct TabContent;

#[derive(Component)]
pub struct TabContentButton;

/// Settings used to construct a tabbed view
pub struct TabbedContentSettings {
    /// Each tab needs a unique identifier
    pub tabs: Vec<String>,
    pub open_tab: usize,
}

/// Construct and spawn a new tabbed content container. Returns the entity for the entire tabbed content and the entity for each tab.
pub fn tabbed_content<T>(
    menu_type: T,
    tab_settings: TabbedContentSettings,
    colors: &CurrentColors,
    commands: &mut Commands,
) -> (Entity, Vec<(Entity, String)>)
where
    T: Component,
{
    let mut tab_entities = Vec::with_capacity(tab_settings.tabs.len());

    let root = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: colors.background().into(),
            focus_policy: FocusPolicy::Block,
            ..default()
        },))
        .insert(menu_type)
        .id();

    let tab_buttons = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                max_height: Val::Percent(15.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                border: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
            border_color: colors.background_light().into(),
            ..default()
        },))
        .id();

    let tab_buttons_inside = commands
        .spawn((NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                flex_direction: FlexDirection::Row,
                padding: UiRect::vertical(Val::Percent(5.0)),
                ..default()
            },
            ..default()
        },))
        .id();

    commands
        .entity(tab_buttons)
        .push_children(&[tab_buttons_inside]);

    let tab_contents = commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(85.0),
                max_height: Val::Percent(85.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Relative,
                ..default()
            },
            ..default()
        },))
        .id();

    for (i, tab) in tab_settings.tabs.iter().enumerate() {
        let display = if i == tab_settings.open_tab {
            Display::DEFAULT
        } else {
            Display::None
        };
        let content = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Relative,
                        display,
                        ..default()
                    },
                    ..default()
                },
                TabContent,
            ))
            .id();

        tab_entities.push((content, tab.clone()));

        let button_style = BasicButtonStyle {
            bundle: Some(TabContentEntity(content)),
            text: tab.clone(),
            font_size: 40.0,
        };
        let button = basic_button(TabContentButton, button_style, commands, colors);
        if i == tab_settings.open_tab {
            commands.entity(button).insert(SelectedButton);
        }
        commands.entity(tab_buttons_inside).push_children(&[button]);
        commands.entity(tab_contents).push_children(&[content]);
    }

    commands
        .entity(root)
        .push_children(&[tab_buttons, tab_contents]);

    (root, tab_entities)
}
