use gpui::{
    relative, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, RenderOnce,
    Styled, Svg, WindowContext,
};

use crate::{Clickable, Disableable, FixedWidth, Toggleable};

use super::button_base::{ButtonAppearance, ButtonBase, ButtonShape, ButtonSize};

#[derive(IntoElement)]
pub struct IconButton {
    base: ButtonBase,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ButtonBase::new(id),
        }
    }

    pub fn icon(mut self, icon: Svg) -> Self {
        self.base = self.base.leading(icon);
        self
    }

    pub fn appearance(mut self, style: ButtonAppearance) -> Self {
        self.base = self.base.appearance(style);
        self
    }

    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.base = self.base.shape(shape);
        self
    }

    pub fn compact(mut self) -> Self {
        self.base = self.base.compact();
        self
    }
}

impl Clickable for IconButton {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.base = self.base.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
        self
    }
}

impl Toggleable for IconButton {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.base = self.base.toggle_state(selected);
        self
    }
}

impl Disableable for IconButton {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl FixedWidth for IconButton {
    fn width(mut self, width: gpui::DefiniteLength) -> Self {
        self.base = self.base.w(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.w(relative(1.));
        self
    }
}

impl Styled for IconButton {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for IconButton {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl From<IconButton> for AnyElement {
    fn from(button: IconButton) -> Self {
        button.into_any_element()
    }
}

impl RenderOnce for IconButton {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        let padding = match self.base.size {
            ButtonSize::Normal => gpui::px(8.),
            ButtonSize::Compact => gpui::px(4.),
        };

        self.base.p(padding)
    }
}
