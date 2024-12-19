use gpui::{px, AbsoluteLength};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BorderRadius {
    None,
    Small,
    Medium,
    Large,
    XLarge,
    Circular,
}

impl BorderRadius {
    pub fn size(&self) -> impl Into<AbsoluteLength> {
        match self {
            BorderRadius::None => px(0.),
            BorderRadius::Small => px(2.),
            BorderRadius::Medium => px(4.),
            BorderRadius::Large => px(6.),
            BorderRadius::XLarge => px(8.),
            BorderRadius::Circular => px(10000.),
        }
    }
}

impl From<BorderRadius> for AbsoluteLength {
    fn from(value: BorderRadius) -> Self {
        value.size().into()
    }
}
