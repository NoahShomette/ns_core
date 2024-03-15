use bevy::{
    app::{Plugin, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{Added, Changed, With, Without},
        removal_detection::RemovedComponents,
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, Children},
    prelude::default,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, Interaction, JustifyContent, Outline, Style, UiRect, Val,
    },
};
use bevy_mod_picking::focus::PickingInteraction;

use crate::ui::colors::CurrentColors;

use super::{DisabledButton, SelectedButton};

pub struct BasicButtonPlugin;

impl Plugin for BasicButtonPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (handle_basic_button_visuals, handle_selected_outlines),
        );
    }
}

#[derive(Component)]
pub struct BasicButton;

fn handle_basic_button_visuals(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (
            Changed<Interaction>,
            With<Button>,
            Without<DisabledButton>,
            With<BasicButton>,
        ),
    >,
    mut children_text_color: Query<&mut Text>,
    colors: Res<CurrentColors>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor::from(colors.highlight());
            }
            Interaction::Hovered => {
                *color = BackgroundColor::from(colors.interactive());
                for &child in children.iter() {
                    if let Ok(mut text) = children_text_color.get_mut(child) {
                        text.sections[0].style.color = colors.light_text();
                    }
                }
            }
            Interaction::None => {
                *color = BackgroundColor::from(colors.background_dark());
                for &child in children.iter() {
                    if let Ok(mut text) = children_text_color.get_mut(child) {
                        text.sections[0].style.color = colors.light_text();
                    }
                }
            }
        }
    }
}

fn handle_selected_outlines(
    mut added_selection_query: Query<
        Entity,
        (Added<SelectedButton>, With<Button>, With<BasicButton>),
    >,
    mut removed_selected_events: RemovedComponents<SelectedButton>,
    colors: Res<CurrentColors>,
    mut commands: Commands,
) {
    for entity in &mut added_selection_query {
        commands.entity(entity).insert(Outline {
            width: Val::Px(3.0),
            offset: Val::Px(0.0),
            color: colors.accent(),
        });
    }

    for entity in &mut removed_selected_events.read() {
        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<Outline>();
        }
    }
}

pub struct BasicButtonStyle<B: Bundle> {
    pub bundle: Option<B>,
    pub text: String,
    pub font_size: f32,
}

impl<B> Default for BasicButtonStyle<B>
where
    B: Bundle,
{
    fn default() -> Self {
        Self {
            bundle: Default::default(),
            text: Default::default(),
            font_size: 40.0,
        }
    }
}

pub fn basic_button<T>(
    button_marker: T,
    button_style: BasicButtonStyle<impl Bundle>,
    commands: &mut Commands,
    colors: &CurrentColors,
) -> Entity
where
    T: Component,
{
    let mut entity = commands.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(button_style.font_size + 10.0),
                margin: UiRect::new(Val::Px(10.0), Val::Px(10.0), Val::Px(10.0), Val::Px(10.0)),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor::from(colors.background()),
            ..Default::default()
        },
        button_marker,
        BasicButton,
        PickingInteraction::default(),
    ));

    entity.with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                &button_style.text,
                TextStyle {
                    font_size: button_style.font_size,
                    color: colors.light_text(),
                    ..default()
                },
            ),
            PickingInteraction::default(),
        ));
    });

    if let Some(bundle) = button_style.bundle {
        entity.insert(bundle);
    };

    entity.id()
}
