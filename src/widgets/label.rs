use gpui::{
    div, prelude::FluentBuilder, relative, AnyElement, App, Div, FontWeight, Hsla, IntoElement,
    ParentElement, RenderOnce, StyleRefinement, Styled, UnderlineStyle, Window,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum LabelSize {
    #[default]
    Default,
    Large,
    Small,
    XSmall,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum LineHeightStyle {
    #[default]
    TextLabel,
    /// Sets the line height to 1.
    UiLabel,
}

#[derive(IntoElement)]
pub struct Label {
    base: Div,
    size: LabelSize,
    weight: Option<FontWeight>,
    line_height_style: LineHeightStyle,
    color: Option<Hsla>,
    strikethrough: bool,
    italic: bool,
    children: Vec<AnyElement>,
    underline: bool,
    single_line: bool,
}

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}

impl Label {
    pub fn new() -> Self {
        Self {
            base: div(),
            size: LabelSize::Default,
            weight: None,
            line_height_style: LineHeightStyle::default(),
            color: None,
            strikethrough: false,
            italic: false,
            children: Vec::new(),
            underline: false,
            single_line: false,
        }
    }

    pub fn size(mut self, size: LabelSize) -> Self {
        self.size = size;
        self
    }

    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn line_height_style(mut self, line_height_style: LineHeightStyle) -> Self {
        self.line_height_style = line_height_style;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.strikethrough = strikethrough;
        self
    }

    pub fn italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn single_line(mut self) -> Self {
        self.single_line = true;
        self
    }
}

impl Styled for Label {
    fn style(&mut self) -> &mut StyleRefinement {
        self.base.style()
    }
}

impl ParentElement for Label {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Label {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.base
            .map(|this| match self.size {
                LabelSize::Large => this.text_lg(),
                LabelSize::Default => this.text_base(),
                LabelSize::Small => this.text_sm(),
                LabelSize::XSmall => this.text_xs(),
            })
            .when_some(self.color, |this, color| this.text_color(color))
            .when_some(self.weight, |this, weight| this.font_weight(weight))
            .when(self.line_height_style == LineHeightStyle::UiLabel, |this| {
                this.line_height(relative(1.))
            })
            .when(self.italic, |this| this.italic())
            .when(self.underline, |mut this| {
                this.text_style()
                    .get_or_insert_with(Default::default)
                    .underline = Some(UnderlineStyle {
                    thickness: gpui::Pixels::from(1.),
                    color: None,
                    wavy: false,
                });
                this
            })
            .when(self.strikethrough, |this| this.line_through())
            .when(self.single_line, |this| this.whitespace_nowrap())
            .children(self.children)
    }
}
