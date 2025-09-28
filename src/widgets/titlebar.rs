use gpui::{
    div, prelude::FluentBuilder, px, AnyElement, App, Div, ElementId, InteractiveElement,
    Interactivity, IntoElement, ParentElement, Pixels, RenderOnce, Rgba,
    StatefulInteractiveElement, Styled, Window,
};

use crate::{h_flex, Brightness, Platform, ThemeProvider};

const TRAFFIC_LIGHT_PADDING: f32 = 71.;

#[derive(IntoElement)]
pub struct TitleBar {
    base: Div,
    children: Vec<AnyElement>,
}

impl RenderOnce for TitleBar {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let height = Self::height(window);
        let platform = Platform::current();

        h_flex()
            .id("titlebar")
            .w_full()
            .h(height)
            .map(|this| {
                if window.is_fullscreen() {
                    this.pl_2()
                } else if platform == Platform::Mac {
                    this.pl(px(TRAFFIC_LIGHT_PADDING))
                } else {
                    this.pl_2()
                }
            })
            .content_stretch()
            .child(
                div()
                    .id("titlebar-content")
                    .flex()
                    .flex_row()
                    .justify_between()
                    .w_full()
                    .when(platform != Platform::Windows, |this| {
                        this.on_click(|event, window, _| {
                            if event.click_count() == 2 {
                                window.zoom_window();
                            }
                        })
                    })
                    .children(self.children),
            )
            .when(!window.is_fullscreen(), |title_bar| match platform {
                Platform::Mac | Platform::Linux => title_bar,
                Platform::Windows => title_bar.child(WindowsWindowControls::new(height)),
            })
    }
}

impl TitleBar {
    pub fn new() -> Self {
        Self {
            base: div(),
            children: vec![],
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn height(cx: &mut Window) -> Pixels {
        (1.75 * cx.rem_size()).max(px(34.))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_cx: &mut Window) -> Pixels {
        px(32.)
    }
}

impl Default for TitleBar {
    fn default() -> Self {
        Self::new()
    }
}

impl InteractiveElement for TitleBar {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for TitleBar {}

impl ParentElement for TitleBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

#[derive(IntoElement)]
pub struct WindowsWindowControls {
    button_height: Pixels,
}

impl WindowsWindowControls {
    pub fn new(button_height: Pixels) -> Self {
        Self { button_height }
    }

    fn get_font() -> &'static str {
        "Segoe Fluent Icons"
    }
}

impl RenderOnce for WindowsWindowControls {
    fn render(self, window: &mut Window, app: &mut App) -> impl IntoElement {
        let brightness = app.theme().brightness();

        let close_button_hover_color = Rgba {
            r: 232.0 / 255.0,
            g: 17.0 / 255.0,
            b: 32.0 / 255.0,
            a: 1.0,
        };

        let button_hover_color = match brightness {
            Brightness::Light => Rgba {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 0.2,
            },
            Brightness::Dark => Rgba {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 0.1,
            },
        };

        div()
            .id("windows-window-controls")
            .font_family(Self::get_font())
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .max_h(self.button_height)
            .min_h(self.button_height)
            .child(WindowsCaptionButton::new(
                "minimize",
                WindowsCaptionButtonIcon::Minimize,
                button_hover_color,
            ))
            .child(WindowsCaptionButton::new(
                "maximize-or-restore",
                if window.is_maximized() {
                    WindowsCaptionButtonIcon::Restore
                } else {
                    WindowsCaptionButtonIcon::Maximize
                },
                button_hover_color,
            ))
            .child(WindowsCaptionButton::new(
                "close",
                WindowsCaptionButtonIcon::Close,
                close_button_hover_color,
            ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum WindowsCaptionButtonIcon {
    Minimize,
    Restore,
    Maximize,
    Close,
}

#[derive(IntoElement)]
struct WindowsCaptionButton {
    id: ElementId,
    icon: WindowsCaptionButtonIcon,
    hover_background_color: Rgba,
}

impl WindowsCaptionButton {
    pub fn new(
        id: impl Into<ElementId>,
        icon: WindowsCaptionButtonIcon,
        hover_background_color: Rgba,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            hover_background_color,
        }
    }
}

impl RenderOnce for WindowsCaptionButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let width = px(36.);

        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .content_center()
            .w(width)
            .h_full()
            .text_size(px(10.0))
            .hover(|style| style.bg(self.hover_background_color))
            .active(|style| {
                let mut active_color = self.hover_background_color;
                active_color.a *= 0.2;

                style.bg(active_color)
            })
            .child(match self.icon {
                WindowsCaptionButtonIcon::Minimize => "\u{e921}",
                WindowsCaptionButtonIcon::Restore => "\u{e923}",
                WindowsCaptionButtonIcon::Maximize => "\u{e922}",
                WindowsCaptionButtonIcon::Close => "\u{e8bb}",
            })
    }
}
