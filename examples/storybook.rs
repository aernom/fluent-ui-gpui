mod assets;
mod stories;

use std::path::PathBuf;

use assets::Assets;
use fluent::*;
use gpui::*;
use stories::*;

#[derive(Debug, PartialEq, Eq)]
enum Story {
    Button,
    Divider,
    Input,
}

impl From<Story> for ElementId {
    fn from(val: Story) -> Self {
        match val {
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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = cx.theme().colors();

        h_flex()
            .size_full()
            .text_color(colors.on_neutral())
            .bg(colors.surface_blur_layer())
            .items_start()
            .children([
                v_flex().pt_10().px_2().w(px(250.)).children([
                    NavItem::new(Story::Button)
                        .label("Buttons")
                        .orientation(Orientation::Horizontal)
                        .toggle_state(self.selected_tab == Story::Button)
                        .on_click(cx.listener(|view, _, _, cx| {
                            view.selected_tab = Story::Button;
                            cx.notify();
                        })),
                    NavItem::new(Story::Divider)
                        .label("Dividers")
                        .orientation(Orientation::Horizontal)
                        .toggle_state(self.selected_tab == Story::Divider)
                        .on_click(cx.listener(|view, _, _, cx| {
                            view.selected_tab = Story::Divider;
                            cx.notify();
                        })),
                    NavItem::new(Story::Input)
                        .label("Inputs")
                        .orientation(Orientation::Horizontal)
                        .toggle_state(self.selected_tab == Story::Input)
                        .on_click(cx.listener(|view, _, _, cx| {
                            view.selected_tab = Story::Input;
                            cx.notify();
                        })),
                    NavItem::new("disabled")
                        .label("Disabled")
                        .orientation(Orientation::Horizontal)
                        .toggle_state(false)
                        .disabled(true),
                ]),
                v_flex()
                    .pt_5()
                    .flex_wrap()
                    .flex_1()
                    .h_full()
                    .bg(colors.surface())
                    .child(
                        h_flex().mb_6().children([
                            NavItem::new(Story::Button)
                                .label("Buttons")
                                .toggle_state(self.selected_tab == Story::Button)
                                .on_click(cx.listener(|view, _, _, cx| {
                                    view.selected_tab = Story::Button;
                                    cx.notify();
                                })),
                            NavItem::new(Story::Divider)
                                .label("Dividers")
                                .toggle_state(self.selected_tab == Story::Divider)
                                .on_click(cx.listener(|view, _, _, cx| {
                                    view.selected_tab = Story::Divider;
                                    cx.notify();
                                })),
                            NavItem::new(Story::Input)
                                .label("Inputs")
                                .toggle_state(self.selected_tab == Story::Input)
                                .on_click(cx.listener(|view, _, _, cx| {
                                    view.selected_tab = Story::Input;
                                    cx.notify();
                                })),
                            NavItem::new("disabled")
                                .label("Disabled")
                                .toggle_state(false)
                                .disabled(true),
                        ]),
                    )
                    .child(match self.selected_tab {
                        Story::Button => buttons_page(),
                        Story::Divider => dividers_page(),
                        Story::Input => div().p_12(), /*.child(InputStory::view(cx))*/
                    }),
            ])
            .child(
                div().absolute().left_0().top_0().right_0().child(
                    TitleBar::new().child(
                        div()
                            .text_sm()
                            .child("This is a custom TitleBar, your content goes here"),
                    ),
                ),
            )
    }
}

fn main() {
    Application::new()
        .with_assets(Assets::from(PathBuf::from("examples/assets")))
        .run(|cx: &mut App| {
            // cx.set_global(Theme::light());
            // cx.set_global(Theme::dark());
            cx.set_global(Theme::system(cx));
            cx.activate(true);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,
                        size(px(1000.0), px(700.0)),
                        cx,
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("AlfaRobot".into()),
                        appears_transparent: true,
                        ..TitlebarOptions::default()
                    }),
                    window_background: WindowBackgroundAppearance::Blurred,
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_cx| Storybook {
                        selected_tab: Story::Button,
                    })
                },
            )
            .unwrap();
        });
}
