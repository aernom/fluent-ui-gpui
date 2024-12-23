use gpui::{
    div, prelude::FluentBuilder, px, relative, rgba, AbsoluteLength, AnyElement, ClickEvent,
    CursorStyle, Div, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, Rgba, StatefulInteractiveElement, Styled, Svg, WindowContext,
};

use crate::{BorderRadius, Clickable, Disableable, FixedWidth, Theme};

#[derive(IntoElement)]
pub(super) struct ButtonBase {
    pub(super) base: Div,
    id: ElementId,
    leading: Option<Svg>,
    trailing: Option<Svg>,
    label: Option<AnyElement>,
    disabled: bool,
    appearance: ButtonAppearance,
    pub(super) selected: bool,
    shape: ButtonShape,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
    cursor_style: CursorStyle,
}

impl ButtonBase {
    pub(super) fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div().flex_shrink_0(),
            id: id.into(),
            leading: None,
            trailing: None,
            label: None,
            disabled: false,
            appearance: ButtonAppearance::default(),
            selected: false,
            shape: ButtonShape::default(),
            on_click: None,
            cursor_style: CursorStyle::PointingHand,
        }
    }

    pub(super) fn label(mut self, label: impl IntoElement) -> Self {
        self.label = Some(label.into_any_element());
        self
    }

    pub(super) fn leading(mut self, leading: Svg) -> Self {
        self.leading = Some(leading);
        self
    }

    pub(super) fn trailing(mut self, trailing: Svg) -> Self {
        self.trailing = Some(trailing);
        self
    }

    pub(super) fn appearance(mut self, style: ButtonAppearance) -> Self {
        self.appearance = style;
        self
    }

    pub(super) fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }
}

impl Clickable for ButtonBase {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl Disableable for ButtonBase {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl FixedWidth for ButtonBase {
    fn width(mut self, width: gpui::DefiniteLength) -> Self {
        self.base = self.base.w(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.w(relative(1.));
        self
    }
}

impl Styled for ButtonBase {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for ButtonBase {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl From<ButtonBase> for AnyElement {
    fn from(button: ButtonBase) -> Self {
        button.into_any_element()
    }
}

impl RenderOnce for ButtonBase {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        self.base
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .px(px(12.))
            .py(px(5.))
            .gap_x(px(8.))
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::NORMAL)
            .rounded(self.shape.radius())
            .border_1()
            .cursor_pointer()
            .map(|this| {
                if self.disabled {
                    let colors = self.appearance.disabled(cx);
                    this.text_color(colors.text)
                        .bg(colors.bg)
                        .border_color(colors.outline)
                        .cursor_not_allowed()
                        .when_some(self.leading, |this, leading| {
                            this.child(leading.w_5().h_5().text_color(colors.text))
                        })
                        .when_some(self.label, |this, label| this.child(label))
                        .when_some(self.trailing, |this, trailing| {
                            this.child(trailing.w_5().h_5().text_color(colors.text))
                        })
                } else {
                    let colors = if self.selected {
                        self.appearance.selected(cx)
                    } else {
                        self.appearance.base(cx)
                    };

                    this.text_color(colors.text)
                        .bg(colors.bg)
                        .border_color(colors.outline)
                        .hover(|this| {
                            let colors = self.appearance.hover(cx);
                            this.text_color(colors.text)
                                .bg(colors.bg)
                                .border_color(colors.outline)
                        })
                        .active(|style| style.opacity(0.8))
                        .when_some(self.leading, |this, leading| {
                            this.child(leading.size_4().text_color(colors.text))
                        })
                        .when_some(self.label, |this, label| this.child(label))
                        .when_some(self.trailing, |this, trailing| {
                            this.child(trailing.size_4().text_color(colors.text))
                        })
                }
            })
    }
}

pub(super) struct ButtonStyle {
    bg: Rgba,
    text: Rgba,
    outline: Rgba,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonAppearance {
    Accent,
    #[default]
    Neutral,
    Subtle,
}

impl ButtonAppearance {
    fn base(&self, cx: &WindowContext) -> ButtonStyle {
        let colors = Theme::of(cx).colors();

        match self {
            ButtonAppearance::Accent => ButtonStyle {
                bg: colors.accent(),
                text: colors.on_accent(),
                outline: rgba(0xffffff00),
            },
            ButtonAppearance::Neutral => ButtonStyle {
                bg: colors.neutral(),
                text: colors.on_neutral(),
                outline: colors.stroke_neutral(),
            },
            ButtonAppearance::Subtle => ButtonStyle {
                bg: colors.subtle(),
                text: colors.on_neutral_variant(),
                outline: rgba(0xffffff00),
            },
        }
    }

    fn hover(&self, cx: &WindowContext) -> ButtonStyle {
        let colors = Theme::of(cx).colors();

        match self {
            ButtonAppearance::Accent => ButtonStyle {
                bg: colors.accent_hover(),
                text: colors.on_accent(),
                outline: rgba(0xffffff00),
            },
            ButtonAppearance::Neutral => ButtonStyle {
                bg: colors.neutral_hover(),
                text: colors.on_neutral(),
                outline: colors.stroke_neutral(),
            },
            ButtonAppearance::Subtle => ButtonStyle {
                bg: colors.subtle_hover(),
                text: colors.on_subtle(),
                outline: rgba(0xffffff00),
            },
        }
    }

    fn disabled(&self, cx: &WindowContext) -> ButtonStyle {
        let colors = Theme::of(cx).colors();

        match self {
            ButtonAppearance::Accent => ButtonStyle {
                bg: colors.accent_disabled(),
                text: colors.on_accent_disabled(),
                outline: rgba(0xffffff00),
            },
            ButtonAppearance::Neutral => ButtonStyle {
                bg: colors.neutral_disabled(),
                text: colors.on_neutral_disabled(),
                outline: colors.stroke_neutral_disabled(),
            },
            ButtonAppearance::Subtle => ButtonStyle {
                bg: colors.subtle(),
                text: colors.on_subtle_disabled(),
                outline: rgba(0xffffff00),
            },
        }
    }

    fn selected(&self, cx: &WindowContext) -> ButtonStyle {
        let colors = Theme::of(cx).colors();

        match self {
            ButtonAppearance::Accent => ButtonStyle {
                bg: colors.neutral_disabled(),
                text: colors.on_neutral_disabled(),
                outline: rgba(0xffffff00),
            },
            ButtonAppearance::Neutral => ButtonStyle {
                bg: colors.neutral_disabled(),
                text: colors.on_neutral_disabled(),
                outline: colors.stroke_neutral_disabled(),
            },
            ButtonAppearance::Subtle => ButtonStyle {
                bg: rgba(0xffffff00),
                text: colors.on_neutral_disabled(),
                outline: rgba(0xffffff00),
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonShape {
    #[default]
    Rounded,
    Circular,
    Square,
}

impl ButtonShape {
    fn radius(&self) -> impl Clone + Into<AbsoluteLength> {
        match self {
            ButtonShape::Rounded => BorderRadius::Medium,
            ButtonShape::Circular => BorderRadius::Circular,
            ButtonShape::Square => BorderRadius::None,
        }
    }
}
