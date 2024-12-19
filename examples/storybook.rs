mod assets;
mod stories;

use std::path::PathBuf;

use assets::Assets;
use gpui::*;
use luna::*;
use stories::*;

#[derive(Debug, PartialEq, Eq)]
enum Story {
    Button,
    Divider,
    Input,
}

impl Into<ElementId> for Story {
    fn into(self) -> ElementId {
        match self {
            Story::Button => "button_story",
            Story::Divider => "divider_story",
            Story::Input => "input_story",
        }
        .into()
    }
}

struct Storybook {
    selected_tab: Story,
}

impl Render for Storybook {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .w_full()
            .h_full()
            .flex_wrap()
            .text_color(cx.theme().colors().on_neutral())
            .bg(cx.theme().colors().surface())
            .child(TitleBar::new().child(div().text_sm().child("This is a custom TitleBar")))
            .child(
                h_flex().mb_6().children([
                    luna::Tab::new(Story::Button, "Buttons", self.selected_tab == Story::Button)
                        .on_click(cx.listener(|view, _, cx| {
                            view.selected_tab = Story::Button;
                            cx.notify();
                        })),
                    luna::Tab::new(
                        Story::Divider,
                        "Dividers",
                        self.selected_tab == Story::Divider,
                    )
                    .on_click(cx.listener(|view, _, cx| {
                        view.selected_tab = Story::Divider;
                        cx.notify();
                    })),
                    luna::Tab::new(Story::Input, "Inputs", self.selected_tab == Story::Input)
                        .on_click(cx.listener(|view, _, cx| {
                            view.selected_tab = Story::Input;
                            cx.notify();
                        })),
                    luna::Tab::new("disabled", "Disabled", false).disabled(true),
                ]),
            )
            .child(match self.selected_tab {
                Story::Button => buttons_page(),
                Story::Divider => dividers_page(),
                Story::Input => div().p_12(), /*.child(InputStory::view(cx))*/
            })
            .child(
                svg()
                    .path("rocket.svg")
                    .w_24()
                    .h_24()
                    .mt_4()
                    .text_color(cx.theme().colors().primary()),
            )
    }
}

fn main() {
    App::new()
        .with_assets(Assets::from(PathBuf::from("examples/assets")))
        .run(|cx: &mut AppContext| {
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
                |cx| {
                    cx.new_view(|_cx| Storybook {
                        selected_tab: Story::Button,
                    })
                },
            )
            .unwrap();
        });
}
