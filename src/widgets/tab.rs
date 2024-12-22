use gpui::{
    div, prelude::FluentBuilder as _, px, AnyElement, Div, ElementId, FontWeight,
    InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement as _, Styled, WindowContext,
};

use crate::Theme;

#[derive(IntoElement)]
pub struct Tab {
    pub base: Div,
    id: ElementId,
    label: SharedString,
    selected: bool,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&bool, &mut WindowContext) + 'static>>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, selected: bool) -> Self {
        Self {
            base: div(),
            id: id.into(),
            label: label.into(),
            selected,
            disabled: false,
            on_click: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&bool, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Tab {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let colors = Theme::of(cx).colors();

        self.base
            .id(self.id)
            .group("tab_area")
            .px_3()
            .py(px(10.))
            .rounded_md()
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .child(self.label)
            .when(self.disabled, |this| {
                this.text_color(colors.on_neutral_disabled())
                    .cursor_not_allowed()
            })
            .when(!self.disabled, |this| {
                this.text_color(match self.selected {
                    true => colors.on_neutral(),
                    false => colors.on_neutral_variant(),
                })
                .cursor_pointer()
                .hover(|style| style.bg(colors.subtle_hover()))
                .active(|style| style.opacity(0.8))
                .child(
                    div()
                        .absolute()
                        .invisible()
                        .h(px(3.))
                        .bottom_0()
                        .left_3()
                        .right_3()
                        .map(|this| match self.selected {
                            true => this.visible().bg(colors.stroke_primary()),
                            false => this.group_hover("tab_area", |this| {
                                this.visible().bg(colors.stroke_neutral())
                            }),
                        }),
                )
            })
            .when_some(
                self.on_click.filter(|_| !self.disabled),
                |this, on_click| this.on_click(move |_, cx| on_click(&true, cx)),
            )
    }
}

impl Styled for Tab {
    fn style(&mut self) -> &mut gpui::StyleRefinement {
        self.base.style()
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl From<Tab> for AnyElement {
    fn from(tab: Tab) -> Self {
        tab.into_any_element()
    }
}
