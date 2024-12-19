use gpui::*;
use luna::*;

struct Playground {}

impl Render for Playground {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();

        v_flex()
            .w_full()
            .h_full()
            .justify_center()
            .text_color(cx.theme().colors().on_neutral())
            .bg(cx.theme().colors().surface())
            .child(
                div().absolute().inset_0().child(
                    TitleBar::new().child(div().text_sm().child("This is a custom TitleBar")),
                ),
            )
            .child(
                div()
                    .size_40()
                    .text_color(colors.on_primary_compound())
                    .bg(colors.primary_compound())
                    .child("Playground"),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.set_global(Theme::system(cx));
        cx.activate(true);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(800.0), px(600.0)),
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some("AlfaRobot".into()),
                    appears_transparent: true,
                    ..TitlebarOptions::default()
                }),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Playground {}),
        )
        .unwrap();
    });
}
