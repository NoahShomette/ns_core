use bevy::app::{Plugin, Update};
use bevy::ecs::component::Component;
use bevy::ecs::event::{EventReader, EventWriter};
use bevy::ecs::query::Added;
use bevy::ecs::system::{Commands, Query};
use bevy::ecs::{entity::Entity, event::Event};
use bevy::hierarchy::Parent;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::ui::{Node, Style, Val};
use bevy_eventlistener::callbacks::ListenerMut;
use bevy_eventlistener::event_listener::{EntityEvent, On};
use bevy_eventlistener::{EntityEvent, EventListenerPlugin};
use bevy_mod_picking::focus::PickingInteraction;

pub struct ScrollCorePlugin;

impl Plugin for ScrollCorePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(EventListenerPlugin::<ScrollEvent>::default());
        app.add_systems(Update, (send_scroll_events, setup_scrollables));
    }
}

#[derive(Clone, Event, EntityEvent)]
pub struct ScrollEvent {
    #[target] // Marks the field of the event that specifies the target entity
    target: Entity,
    pub x: f32,
    pub y: f32,
    pub scroll_amount: MouseScrollUnit,
}

#[derive(Component, Default)]
pub struct Scrollable {
    position: f32,
}

pub fn scroll(
    mut event: ListenerMut<ScrollEvent>,
    mut query_list: Query<(&mut Scrollable, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    let Ok((mut scrolling_list, mut style, parent, list_node)) = query_list.get_mut(event.target)
    else {
        return;
    };

    let items_height = list_node.size().y;
    let container_height = query_node.get(parent.get()).unwrap().size().y;

    let max_scroll = (items_height - container_height).max(0.);

    let dy = match event.scroll_amount {
        MouseScrollUnit::Line => event.y * 20.,
        MouseScrollUnit::Pixel => event.y,
    };

    scrolling_list.position += dy;
    scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);

    // if the current scrollable hasnt reached the maximum on either side then we stop the event propogating farther
    if scrolling_list.position >= -max_scroll || scrolling_list.position <= 0. {
        event.stop_propagation();
    }
    style.top = Val::Px(scrolling_list.position);
}

fn send_scroll_events(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut scroll_events: EventWriter<ScrollEvent>,
    query_list: Query<(Entity, &PickingInteraction)>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        for (entity, interaction) in &query_list {
            if interaction == &PickingInteraction::Hovered {
                scroll_events.send(ScrollEvent {
                    target: entity,
                    x: mouse_wheel_event.x,
                    y: mouse_wheel_event.y,
                    scroll_amount: mouse_wheel_event.unit,
                });
            }
        }
    }
}

fn setup_scrollables(query: Query<Entity, Added<On<ScrollEvent>>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).insert(Scrollable::default());
    }
}
