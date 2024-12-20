use gpui::Styled;

pub trait StyledExt: Styled + Sized {
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }
}

impl<S: Styled> StyledExt for S {}
