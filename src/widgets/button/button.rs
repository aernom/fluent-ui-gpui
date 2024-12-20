use gpui::{
    relative, AnyElement, ClickEvent, ElementId, InteractiveElement, IntoElement, RenderOnce,
    Styled, Svg, WindowContext,
};

use crate::{Clickable, Disableable, FixedWidth};

use super::button_base::{ButtonAppearance, ButtonBase, ButtonShape};

#[derive(IntoElement)]
pub struct Button {
    base: ButtonBase,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ButtonBase::new(id),
        }
    }

    pub fn label(mut self, label: impl IntoElement) -> Self {
        self.base = self.base.label(label.into_any_element());
        self
    }

    pub fn leading(mut self, leading: Svg) -> Self {
        self.base = self.base.leading(leading);
        self
    }

    pub fn trailing(mut self, trailing: Svg) -> Self {
        self.base = self.base.trailing(trailing);
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
}

impl Clickable for Button {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.base = self.base.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl FixedWidth for Button {
    fn width(mut self, width: gpui::DefiniteLength) -> Self {
        self.base = self.base.w(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.w(relative(1.));
        self
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Button {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl From<Button> for AnyElement {
    fn from(button: Button) -> Self {
        button.into_any_element()
    }
}

impl RenderOnce for Button {
    fn render(self, _: &mut WindowContext) -> impl IntoElement {
        self.base
    }
}
