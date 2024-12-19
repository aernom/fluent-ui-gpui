use gpui::{ClickEvent, CursorStyle, WindowContext};

pub trait Clickable {
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self;

    fn cursor_style(self, cursor_style: CursorStyle) -> Self;
}
