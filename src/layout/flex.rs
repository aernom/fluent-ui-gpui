use gpui::{div, Div, Styled};

pub fn h_flex() -> Div {
    div().flex().flex_row().items_center()
}

pub fn v_flex() -> Div {
    div().flex().flex_col().items_center()
}

pub fn center() -> Div {
    div().flex().items_center().justify_center()
}
