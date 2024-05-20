use bevy::{app::Plugin, ecs::system::Resource, render::color::Color};

pub struct GameColorsPlugin {
    pub custom_colors: Option<CurrentColors>,
}

impl Plugin for GameColorsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if let Some(custom_colors) = self.custom_colors {
            app.insert_resource(custom_colors);
        } else {
            app.init_resource::<CurrentColors>();
        }
    }
}

#[derive(Resource, Default, Clone, Copy)]
pub struct CurrentColors(GameColorPalette);

impl CurrentColors {
    /// Dark colored text
    pub fn dark_text(&self) -> Color {
        self.0.dark_text()
    }
    /// Light colored text
    pub fn light_text(&self) -> Color {
        self.0.light_text()
    }
    /// A standard background color
    pub fn background(&self) -> Color {
        self.0.background()
    }
    /// A dark colored background color
    pub fn background_dark(&self) -> Color {
        self.0.background_dark()
    }
    /// A light colored background color
    pub fn background_light(&self) -> Color {
        self.0.background_light()
    }
    /// An accent color that is alternatively used with highlight in order to direct attention. Should not by itself denote interactivity but should accent the ui
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

#[derive(Resource, Clone, Copy)]
pub struct GameColorPalette {
    pub dark_text: Color,
    pub light_text: Color,
    pub background: Color,
    pub background_light: Color,
    pub background_dark: Color,
    pub accent: Color,
    pub highlight: Color,
    pub interactive: Color,
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
