mod colors;
pub use colors::*;

use gpui::{AppContext, Global, WindowAppearance};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Brightness {
    Dark,
    Light,
}

pub struct Theme {
    brightness: Brightness,
    colors: ColorScheme,
}

impl Theme {
    pub fn system(cx: &AppContext) -> Self {
        match cx.window_appearance() {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Self::light(),
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Self::dark(),
        }
    }

    pub fn dark() -> Self {
        Self {
            brightness: Brightness::Dark,
            colors: ColorScheme::dark(),
        }
    }

    pub fn light() -> Self {
        Self {
            brightness: Brightness::Light,
            colors: ColorScheme::light(),
        }
    }

    pub fn of(cx: &AppContext) -> &Self {
        cx.global::<Self>()
    }

    pub fn brightness(&self) -> &Brightness {
        &self.brightness
    }

    pub fn colors(&self) -> &ColorScheme {
        &self.colors
    }
}

impl Global for Theme {}

pub trait ThemeProvider {
    fn theme(&self) -> &Theme;
}

impl<T> ThemeProvider for T
where
    T: Deref<Target = gpui::AppContext>,
{
    fn theme(&self) -> &Theme {
        Theme::of(self.deref())
    }
}
