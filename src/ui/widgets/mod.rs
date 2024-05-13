use bevy::app::Plugin;

use self::{
    button::ButtonCorePlugin, modal::ModalPlugin, scroll::ScrollCorePlugin,
    tabbed_content::TabbedContentPlugin,
};

pub mod button;
pub mod modal;
pub mod scroll;
pub mod scroll_container;
pub mod tabbed_content;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            ModalPlugin,
            ButtonCorePlugin,
            TabbedContentPlugin,
            ScrollCorePlugin,
        ));
    }
}
