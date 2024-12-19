use gpui::{rgb, rgba, Rgba};

pub struct ColorScheme {
    neutral: Rgba,
    neutral_hover: Rgba,
    neutral_disabled: Rgba,
    on_neutral: Rgba,
    on_neutral_variant: Rgba,
    on_neutral_disabled: Rgba,
    on_primary: Rgba,
    on_primary_compound: Rgba,
    primary: Rgba,
    primary_hover: Rgba,
    primary_compound: Rgba,
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
            neutral: rgb(0xffffff),
            neutral_hover: rgb(0xf5f5f5),
            neutral_disabled: rgb(0xf0f0f0),
            on_neutral: rgb(0x242424),
            on_neutral_variant: rgb(0x424242),
            on_neutral_disabled: rgb(0xbdbdbd),
            on_primary: rgb(0xffffff),
            on_primary_compound: rgb(0xffffff),
            primary: rgb(0x0f6cbd),
            primary_hover: rgb(0x115ea3),
            primary_compound: rgb(0x0f6cbd),
            primary_stroke: rgb(0x0f6cbd),
            stroke_neutral: rgb(0xd1d1d1),
            stroke_neutral_hover: rgb(0xc7c7c7),
            stroke_neutral_disabled: rgb(0xe0e0e0),
            stroke_neutral_dim: rgb(0xe0e0e0),
            stroke_neutral_subtle: rgb(0xf0f0f0),
            subtle: rgba(0x00000000),
            subtle_hover: rgb(0xf5f5f5),
            surface: rgb(0xfafafa),
        }
    }

    pub fn dark() -> Self {
        Self {
            neutral: rgb(0x292929),
            neutral_hover: rgb(0x3d3d3d),
            neutral_disabled: rgb(0x141414),
            on_neutral: rgb(0xffffff),
            on_neutral_variant: rgb(0xd6d6d6),
            on_neutral_disabled: rgb(0x5c5c5c),
            on_primary: rgb(0xffffff),
            on_primary_compound: rgb(0x000000),
            primary: rgb(0x115ea3),
            primary_hover: rgb(0x0f6cbd),
            primary_compound: rgb(0x479ef5),
            primary_stroke: rgb(0x479ef5),
            stroke_neutral: rgb(0x666666),
            stroke_neutral_hover: rgb(0x757575),
            stroke_neutral_disabled: rgb(0x424242),
            stroke_neutral_dim: rgb(0x525252),
            stroke_neutral_subtle: rgb(0x3d3d3d),
            subtle: rgba(0x00000000),
            subtle_hover: rgb(0x383838),
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

    pub fn primary_compound(&self) -> Rgba {
        self.primary_compound
    }

    pub fn primary_stroke(&self) -> Rgba {
        self.primary_stroke
    }

    pub fn on_primary(&self) -> Rgba {
        self.on_primary
    }

    pub fn on_primary_compound(&self) -> Rgba {
        self.on_primary_compound
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
