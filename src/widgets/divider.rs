use gpui::{
    div, prelude::FluentBuilder, px, Axis, Div, Hsla, IntoElement, RenderOnce, SharedString,
    StyleRefinement, Styled, WindowContext,
};

use crate::ThemeProvider;

#[derive(IntoElement)]
pub struct Divider {
    base: Div,
    label: Option<SharedString>,
    axis: Axis,
    style: DividerStyle,
}

impl Divider {
    pub fn vertical() -> Self {
        Self {
            base: div(),
            axis: Axis::Vertical,
            label: None,
            style: DividerStyle::default(),
        }
    }

    pub fn horizontal() -> Self {
        Self {
            base: div(),
            axis: Axis::Horizontal,
            label: None,
            style: DividerStyle::default(),
        }
    }

    pub fn style(mut self, style: DividerStyle) -> Self {
        self.style = style;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Styled for Divider {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl RenderOnce for Divider {
    fn render(self, cx: &mut WindowContext) -> impl gpui::IntoElement {
        self.base
            .map(|this| match self.axis {
                Axis::Vertical => this.h_full().w(px(1.)),
                Axis::Horizontal => this.w_full().h(px(1.)),
            })
            .bg(self.style.fill(cx))
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum DividerStyle {
    #[default]
    Default,
    Subtle,
    Strong,
    Primary,
    Custom(Hsla),
}

impl DividerStyle {
    fn fill(&self, cx: &WindowContext) -> Hsla {
        let colors = cx.theme().colors();

        match self {
            DividerStyle::Default => colors.neutral_stroke_dim().into(),
            DividerStyle::Subtle => colors.neutral_stroke_subtle().into(),
            DividerStyle::Strong => colors.neutral_stroke().into(),
            DividerStyle::Primary => colors.primary_stroke().into(),
            DividerStyle::Custom(hsla) => *hsla,
        }
    }
}
