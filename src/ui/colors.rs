use bevy::{app::Plugin, ecs::system::Resource, render::color::Color};

pub struct GameColorsPlugin;

impl Plugin for GameColorsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CurrentColors>();
    }
}

#[derive(Resource, Default)]
pub struct CurrentColors(GameColorPalette);

impl CurrentColors {
    pub fn dark_text(&self) -> Color {
        self.0.dark_text()
    }
    pub fn light_text(&self) -> Color {
        self.0.light_text()
    }
    pub fn background(&self) -> Color {
        self.0.background()
    }
    pub fn background_dark(&self) -> Color {
        self.0.background_dark()
    }
    pub fn background_light(&self) -> Color {
        self.0.background_light()
    }

    /// An accent color that is alternatively used with highlight in order to direct attention
    pub fn accent(&self) -> Color {
        self.0.accent()
    }
    /// Color used to highlight something, current actions, something new, etc
    pub fn highlight(&self) -> Color {
        self.0.highlight()
    }
    /// Color used to denote interactive objcets
    pub fn interactive(&self) -> Color {
        self.0.interactive()
    }
}

pub const MODAL_TEXT: Color = Color::rgb(0.12, 0.15, 0.10);
pub const MODAL_BACKGROUND: Color = Color::rgb(0.79, 0.72, 0.65);
pub const MODAL_ALTERNATE: Color = Color::rgb(0.69, 0.62, 0.55);
pub const LIGHT_PURLE: Color = Color::rgb(0.50, 0.45, 0.60);

pub struct GameColorPalette {
    dark_text: Color,
    light_text: Color,
    background: Color,
    background_light: Color,
    background_dark: Color,
    accent: Color,
    highlight: Color,
    interactive: Color,
}

impl Default for GameColorPalette {
    fn default() -> Self {
        GameColorPalette::dark()
    }
}

impl GameColorPalette {
    pub fn dark() -> GameColorPalette {
        Self {
            dark_text: Color::rgb(0.12, 0.15, 0.10),
            light_text: Color::hex("#DAC4AF").unwrap(),
            background: Color::hex("#474340").unwrap(),
            background_light: Color::hex("#988E84").unwrap(),
            background_dark: Color::hex("#6F5A44").unwrap(),
            accent: Color::hex("#F3E9C6").unwrap(),
            highlight: Color::hex("33476C").unwrap(), //Color::rgb(0.07, 0.36, 0.62),
            interactive: Color::hex("#8E5D5D").unwrap(),
        }
    }

    pub fn light() -> GameColorPalette {
        Self {
            dark_text: Default::default(),
            light_text: Default::default(),
            background: Default::default(),
            background_light: Default::default(),
            background_dark: Default::default(),
            accent: Default::default(),
            highlight: Color::rgb(0.07, 0.36, 0.62),
            interactive: Default::default(),
        }
    }

    pub fn dark_text(&self) -> Color {
        self.dark_text
    }
    pub fn light_text(&self) -> Color {
        self.light_text
    }
    pub fn background(&self) -> Color {
        self.background
    }
    pub fn background_light(&self) -> Color {
        self.background_light
    }
    pub fn accent(&self) -> Color {
        self.accent
    }
    pub fn background_dark(&self) -> Color {
        self.background_dark
    }
    pub fn highlight(&self) -> Color {
        self.highlight
    }
    pub fn interactive(&self) -> Color {
        self.interactive
    }
}
