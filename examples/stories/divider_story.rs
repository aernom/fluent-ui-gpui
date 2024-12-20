use fluent::{v_flex, Divider, DividerStyle};
use gpui::{rgb, Div, ParentElement, Styled};

pub fn dividers_page() -> Div {
    v_flex().w_full().gap_4().children([
        Divider::horizontal(),
        Divider::horizontal().style(DividerStyle::Subtle),
        Divider::horizontal().style(DividerStyle::Strong),
        Divider::horizontal().style(DividerStyle::Primary),
        Divider::horizontal().style(DividerStyle::Custom(rgb(0xc239b3).into())),
    ])
}
