use gpui::{rgb, rgba, Rgba};

pub struct ColorScheme {
    neutral: Rgba,
    neutral_hover: Rgba,
    neutral_disabled: Rgba,
    neutral_selected: Rgba,
    on_neutral: Rgba,
    on_neutral_variant: Rgba,
    on_neutral_disabled: Rgba,
    on_primary: Rgba,
    primary: Rgba,
    primary_hover: Rgba,
    primary_stroke: Rgba,
    stroke_neutral: Rgba,
    stroke_neutral_hover: Rgba,
    stroke_neutral_disabled: Rgba,
    stroke_neutral_dim: Rgba,
    stroke_neutral_subtle: Rgba,
    subtle: Rgba,
    subtle_hover: Rgba,
    surface: Rgba,
}

impl ColorScheme {
    pub fn light() -> Self {
        Self {
            neutral: rgba(0xffffffb3),
            neutral_hover: rgba(0xf9f9f980),
            neutral_disabled: rgba(0x00000038),
            neutral_selected: rgb(0xebebeb),
            on_neutral: rgb(0x242424),
            on_neutral_variant: rgb(0x424242),
            on_neutral_disabled: rgb(0xffffff),
            on_primary: rgb(0xffffff),
            primary: rgb(0x0060b8),
            primary_hover: rgb(0x186fbf),
            primary_stroke: rgb(0x0060b8),
            stroke_neutral: rgb(0xd1d1d1),
            stroke_neutral_hover: rgb(0xc7c7c7),
            stroke_neutral_disabled: rgb(0xe0e0e0),
            stroke_neutral_dim: rgb(0xe0e0e0),
            stroke_neutral_subtle: rgb(0xf0f0f0),
            subtle: rgba(0x00000000),
            subtle_hover: rgba(0x0000000a),
            surface: rgb(0xfafafa),
        }
    }

    pub fn dark() -> Self {
        Self {
            neutral: rgba(0xffffff0f),
            neutral_hover: rgba(0xffffff14),
            neutral_disabled: rgba(0xffffff34),
            neutral_selected: rgb(0x383838),
            on_neutral: rgb(0xffffff),
            on_neutral_variant: rgb(0xd6d6d6),
            on_neutral_disabled: rgba(0xffffff87),
            on_primary: rgb(0x000000),
            primary: rgb(0x61ccff),
            primary_hover: rgb(0x59bce7),
            primary_stroke: rgb(0x61ccff),
            stroke_neutral: rgb(0x666666),
            stroke_neutral_hover: rgb(0x757575),
            stroke_neutral_disabled: rgb(0x424242),
            stroke_neutral_dim: rgb(0x525252),
            stroke_neutral_subtle: rgb(0x3d3d3d),
            subtle: rgba(0x00000000),
            subtle_hover: rgba(0xffffff0f),
            surface: rgb(0x1c1c1c),
        }
    }

    pub fn neutral(&self) -> Rgba {
        self.neutral
    }

    pub fn neutral_hover(&self) -> Rgba {
        self.neutral_hover
    }

    pub fn neutral_disabled(&self) -> Rgba {
        self.neutral_disabled
    }

    pub fn neutral_selected(&self) -> Rgba {
        self.neutral_selected
    }

    pub fn neutral_stroke(&self) -> Rgba {
        self.stroke_neutral
    }

    pub fn neutral_stroke_hover(&self) -> Rgba {
        self.stroke_neutral_hover
    }

    pub fn neutral_stroke_disabled(&self) -> Rgba {
        self.stroke_neutral_disabled
    }

    pub fn neutral_stroke_dim(&self) -> Rgba {
        self.stroke_neutral_dim
    }

    pub fn neutral_stroke_subtle(&self) -> Rgba {
        self.stroke_neutral_subtle
    }

    pub fn on_neutral(&self) -> Rgba {
        self.on_neutral
    }

    pub fn on_neutral_variant(&self) -> Rgba {
        self.on_neutral_variant
    }

    pub fn on_neutral_disabled(&self) -> Rgba {
        self.on_neutral_disabled
    }

    pub fn primary(&self) -> Rgba {
        self.primary
    }

    pub fn primary_hover(&self) -> Rgba {
        self.primary_hover
    }

    pub fn primary_stroke(&self) -> Rgba {
        self.primary_stroke
    }

    pub fn on_primary(&self) -> Rgba {
        self.on_primary
    }

    pub fn subtle(&self) -> Rgba {
        self.subtle
    }

    pub fn subtle_hover(&self) -> Rgba {
        self.subtle_hover
    }

    pub fn surface(&self) -> Rgba {
        self.surface
    }
}
