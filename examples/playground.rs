use std::path::PathBuf;

use fluent::*;
use gpui::*;

struct Playground {
    files: Vec<PathBuf>,
    selected: bool,
}

impl Playground {
    fn new() -> Self {
        Self {
            files: vec![],
            selected: false,
        }
    }
}

impl Render for Playground {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();
        let label = match self.selected {
            true => "On",
            false => "Off",
        };

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
                    .id("click me!")
                    .size_40()
                    .text_color(colors.on_accent())
                    .bg(colors.accent())
                    .child("Playground")
                    .on_click(cx.listener(|_, _, cx| {
                        let paths = cx.prompt_for_paths(PathPromptOptions {
                            files: true,
                            directories: false,
                            multiple: true,
                        });

                        cx.spawn(|this, mut cx| async move {
                            if let Ok(Ok(Some(paths))) = paths.await {
                                this.upgrade()
                                    .unwrap()
                                    .update(&mut cx, |this, _| {
                                        this.files.extend(paths);
                                    })
                                    .ok();
                            }
                        })
                        .detach();
                    })),
            )
            .child(
                self.files
                    .iter()
                    .fold(v_flex().items_start(), |stack, file| {
                        stack.child(file.display().to_string())
                    }),
            )
            .child(div().p_12().child(if self.selected {
                "SELECTED!"
            } else {
                "NOT SELECTED!"
            }))
            .child(
                h_flex().gap_4().children([
                    Button::new("toggle button")
                        .mt_4()
                        .label(label)
                        .toggle_state(self.selected)
                        .on_click(cx.listener(|view, _, cx| {
                            view.selected = !view.selected;
                            cx.notify();
                        })),
                    Button::new("toggle button 2")
                        .mt_4()
                        .label(label)
                        .appearance(ButtonAppearance::Accent)
                        .toggle_state(self.selected)
                        .on_click(cx.listener(|view, _, cx| {
                            view.selected = !view.selected;
                            cx.notify();
                        })),
                    Button::new("toggle button 3")
                        .mt_4()
                        .label(label)
                        .appearance(ButtonAppearance::Subtle)
                        .toggle_state(self.selected)
                        .on_click(cx.listener(|view, _, cx| {
                            view.selected = !view.selected;
                            cx.notify();
                        })),
                ]),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.set_global(Theme::light());
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
            |cx| cx.new_view(|_cx| Playground::new()),
        )
        .unwrap();
    });
}
