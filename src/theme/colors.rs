use gpui::{rgb, rgba, Rgba};

pub struct ColorScheme {
    // backgrounds
    neutral: Rgba,
    neutral_hover: Rgba,
    neutral_disabled: Rgba,
    neutral_selected: Rgba,
    accent: Rgba,
    accent_hover: Rgba,
    accent_disabled: Rgba,
    accent_selected: Rgba,
    subtle: Rgba,
    subtle_hover: Rgba,
    subtle_disabled: Rgba,
    subtle_selected: Rgba,
    surface: Rgba,
    // foregrounds
    on_neutral: Rgba,
    on_neutral_variant: Rgba,
    on_neutral_disabled: Rgba,
    on_accent: Rgba,
    on_accent_disabled: Rgba,
    on_subtle: Rgba,
    on_subtle_disabled: Rgba,
    // strokes
    stroke_neutral: Rgba,
    stroke_neutral_disabled: Rgba,
    stroke_neutral_dim: Rgba,
    stroke_neutral_subtle: Rgba,
    stroke_accent: Rgba,
}

impl ColorScheme {
    pub fn light() -> Self {
        Self {
            // backgrounds
            neutral: rgba(0xffffffb3),
            neutral_hover: rgba(0xf9f9f980),
            neutral_disabled: rgba(0xf9f9f94d),
            neutral_selected: rgb(0x0060b8),
            accent: rgb(0x0060b8),
            accent_hover: rgb(0x186fbf),
            accent_disabled: rgba(0x00000038),
            accent_selected: rgb(0x00), // TODO
            subtle: rgba(0x00000000),
            subtle_hover: rgba(0x0000000a),
            subtle_disabled: rgba(0x00000000),
            subtle_selected: rgb(0x00), // TODO
            surface: rgb(0xfafafa),
            // foregrounds
            on_neutral: rgba(0x000000e3),
            on_neutral_variant: rgb(0x424242),
            on_neutral_disabled: rgba(0x0000005c),
            on_accent: rgb(0xffffff),
            on_accent_disabled: rgb(0xffffff),
            on_subtle: rgba(0x000000e3),
            on_subtle_disabled: rgba(0x0000005c),
            // strokes
            stroke_neutral: rgba(0x00000029),
            stroke_neutral_disabled: rgba(0x0000000f),
            stroke_neutral_dim: rgb(0xe0e0e0),
            stroke_neutral_subtle: rgb(0xf0f0f0),
            stroke_accent: rgb(0x0060b8),
        }
    }

    pub fn dark() -> Self {
        Self {
            // backgrounds
            neutral: rgba(0xffffff0f),
            neutral_hover: rgba(0xffffff14),
            neutral_disabled: rgba(0xffffff0a),
            neutral_selected: rgb(0x61ccff),
            accent: rgb(0x61ccff),
            accent_hover: rgb(0x59bce7),
            accent_disabled: rgba(0xffffff29),
            accent_selected: rgb(0x00), // TODO
            subtle: rgba(0x00000000),
            subtle_hover: rgba(0xffffff0f),
            subtle_disabled: rgba(0x00000000),
            subtle_selected: rgb(0x00), // TODO
            surface: rgb(0x1c1c1c),
            // foregrounds
            on_neutral: rgb(0xffffff),
            on_neutral_variant: rgb(0xd6d6d6),
            on_neutral_disabled: rgba(0xffffff5c),
            on_accent: rgb(0x000000),
            on_accent_disabled: rgba(0xffffff87),
            on_subtle: rgb(0xffffff),
            on_subtle_disabled: rgba(0xffffff5c),
            // strokes
            stroke_neutral: rgba(0xffffff17),
            stroke_neutral_disabled: rgba(0xffffff12),
            stroke_neutral_dim: rgb(0x525252),
            stroke_neutral_subtle: rgb(0x3d3d3d),
            stroke_accent: rgb(0x61ccff),
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

    pub fn accent(&self) -> Rgba {
        self.accent
    }

    pub fn accent_hover(&self) -> Rgba {
        self.accent_hover
    }

    pub fn accent_disabled(&self) -> Rgba {
        self.accent_disabled
    }

    pub fn accent_selected(&self) -> Rgba {
        self.accent_selected
    }

    pub fn subtle(&self) -> Rgba {
        self.subtle
    }

    pub fn subtle_hover(&self) -> Rgba {
        self.subtle_hover
    }

    pub fn subtle_disabled(&self) -> Rgba {
        self.subtle_disabled
    }

    pub fn subtle_selected(&self) -> Rgba {
        self.subtle_selected
    }

    pub fn surface(&self) -> Rgba {
        self.surface
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

    pub fn on_accent(&self) -> Rgba {
        self.on_accent
    }

    pub fn on_accent_disabled(&self) -> Rgba {
        self.on_accent_disabled
    }

    pub fn on_subtle(&self) -> Rgba {
        self.on_subtle
    }

    pub fn on_subtle_disabled(&self) -> Rgba {
        self.on_subtle_disabled
    }

    pub fn stroke_neutral(&self) -> Rgba {
        self.stroke_neutral
    }

    pub fn stroke_neutral_disabled(&self) -> Rgba {
        self.stroke_neutral_disabled
    }

    pub fn stroke_neutral_dim(&self) -> Rgba {
        self.stroke_neutral_dim
    }

    pub fn stroke_neutral_subtle(&self) -> Rgba {
        self.stroke_neutral_subtle
    }

    pub fn stroke_accent(&self) -> Rgba {
        self.stroke_accent
    }
}
