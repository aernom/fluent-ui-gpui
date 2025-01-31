use gpui::{ClickEvent, CursorStyle, Window};

pub trait Clickable {
    fn on_click(self, handler: impl Fn(&ClickEvent, &mut Window) + 'static) -> Self;

    fn cursor_style(self, cursor_style: CursorStyle) -> Self;
}
