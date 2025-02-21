use gpui::{
    div, prelude::FluentBuilder as _, px, relative, AnyElement, App, ClickEvent, CursorStyle, Div,
    ElementId, FontWeight, InteractiveElement, IntoElement, MouseButton, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled, Window,
};

use crate::{Clickable, Disableable, FixedWidth, OnClickHandler, Theme, Toggleable};

#[derive(IntoElement)]
pub struct NavItem {
    pub base: Div,
    id: ElementId,
    label: SharedString,
    orientation: Orientation,
    selected: bool,
    cursor_style: CursorStyle,
    disabled: bool,
    on_click: Option<OnClickHandler>,
}

impl NavItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            label: "Nav Item".into(),
            orientation: Orientation::Vertical,
            selected: false,
            cursor_style: CursorStyle::PointingHand,
            disabled: false,
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }
}

impl Clickable for NavItem {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl Disableable for NavItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Toggleable for NavItem {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl FixedWidth for NavItem {
    fn width(mut self, width: gpui::DefiniteLength) -> Self {
        self.base = self.base.w(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.w(relative(1.));
        self
    }
}

impl Styled for NavItem {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for NavItem {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl From<NavItem> for AnyElement {
    fn from(tab: NavItem) -> Self {
        tab.into_any_element()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl RenderOnce for NavItem {
    fn render(self, _: &mut Window, app: &mut App) -> impl IntoElement {
        let colors = Theme::of(app).colors();

        self.base
            .id(self.id)
            .group("tab_area")
            .px_3()
            .py(match self.orientation {
                Orientation::Horizontal => px(8.),
                Orientation::Vertical => px(10.),
            })
            .rounded_md()
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::NORMAL)
            .child(self.label)
            .when(self.disabled, |this| {
                this.text_color(colors.on_neutral_disabled())
                    .cursor_not_allowed()
            })
            .when(!self.disabled, |this| {
                this.text_color(colors.on_neutral())
                    .cursor(self.cursor_style)
                    .hover(|style| style.bg(colors.subtle_hover()))
                    .active(|style| style.opacity(0.8))
                    .child(
                        div()
                            .absolute()
                            .invisible()
                            .rounded(px(1.5))
                            .map(|this| match self.orientation {
                                Orientation::Horizontal => {
                                    this.w(px(3.)).left_0().top(px(10.)).bottom(px(10.))
                                }
                                Orientation::Vertical => {
                                    this.h(px(3.)).bottom_0().left_3().right_3()
                                }
                            })
                            .map(|this| match self.selected {
                                true => this.visible().bg(colors.stroke_accent()),
                                false => this.group_hover("tab_area", |this| {
                                    this.visible().bg(colors.stroke_neutral())
                                }),
                            }),
                    )
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| {
                    this.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                        .on_click(move |event, window, app| {
                            app.stop_propagation();
                            (on_click)(event, window, app)
                        })
                },
            )
    }
}
