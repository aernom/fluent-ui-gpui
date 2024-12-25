use fluent::{h_flex, v_flex, Button, ButtonAppearance, ButtonShape, Disableable, IconButton};
use gpui::{div, red, svg, Div, ParentElement, Styled};

pub fn buttons_page() -> Div {
    v_flex()
        .gap_4()
        .child(
            h_flex().gap_2().children([
                Button::new(1)
                    .label("Primary")
                    .appearance(ButtonAppearance::Accent),
                Button::new(4)
                    .label("Primary Disabled")
                    .appearance(ButtonAppearance::Accent)
                    .disabled(true),
            ]),
        )
        .child(
            h_flex().gap_2().children([
                Button::new(2)
                    .label("Outline")
                    .appearance(ButtonAppearance::Neutral),
                Button::new(5)
                    .label("Outline Disabled")
                    .appearance(ButtonAppearance::Neutral)
                    .disabled(true),
            ]),
        )
        .child(
            h_flex().gap_2().children([
                Button::new(3)
                    .label("Subtle")
                    .appearance(ButtonAppearance::Subtle),
                Button::new(6)
                    .label("Subtle Disabled")
                    .appearance(ButtonAppearance::Subtle)
                    .disabled(true),
            ]),
        )
        .child(
            h_flex().gap_2().children([
                Button::new(7).label("Rounded (Default)"),
                Button::new(8)
                    .label("Circular")
                    .appearance(ButtonAppearance::Neutral)
                    .shape(ButtonShape::Circular),
                Button::new(9)
                    .label("Square")
                    .appearance(ButtonAppearance::Neutral)
                    .shape(ButtonShape::Square),
            ]),
        )
        .child(
            h_flex().gap_2().children([
                Button::new(46)
                    .label("Accent Compact")
                    .appearance(ButtonAppearance::Accent)
                    .compact(),
                Button::new(45).label("Default Compact").compact(),
                Button::new(47)
                    .label("Subtle Compact")
                    .appearance(ButtonAppearance::Subtle)
                    .compact(),
            ]),
        )
        .child(
            h_flex().flex_wrap().gap_2().children([
                Button::new(10)
                    .leading(svg().path("send.svg"))
                    .label("Cookie"),
                Button::new(11)
                    .trailing(svg().path("send.svg"))
                    .label("Cookie"),
                Button::new(12)
                    .leading(svg().path("send.svg"))
                    .trailing(svg().path("send.svg"))
                    .label("Cookie"),
                Button::new(13).leading(svg().path("send.svg")).label(
                    v_flex()
                        .items_start()
                        .child("Buttons can have")
                        .child(div().text_xs().text_color(red()).child("SECONDARY CONTENT")),
                ),
            ]),
        )
        .child(
            h_flex().flex_wrap().gap_2().children([
                IconButton::new(14).icon(svg().path("send.svg")),
                IconButton::new(15)
                    .icon(svg().path("send.svg"))
                    .appearance(ButtonAppearance::Subtle),
                IconButton::new(16)
                    .icon(svg().path("send.svg"))
                    .appearance(ButtonAppearance::Accent),
                IconButton::new(17)
                    .icon(svg().path("send.svg"))
                    .appearance(ButtonAppearance::Accent)
                    .shape(ButtonShape::Square),
                IconButton::new(18)
                    .icon(svg().path("send.svg"))
                    .appearance(ButtonAppearance::Accent)
                    .shape(ButtonShape::Circular),
            ]),
        )
        .child(
            h_flex().flex_wrap().gap_2().children([
                IconButton::new(19).icon(svg().path("send.svg")).compact(),
                IconButton::new(20)
                    .icon(svg().path("send.svg"))
                    .appearance(ButtonAppearance::Subtle)
                    .compact(),
                IconButton::new(21)
                    .icon(svg().path("send.svg"))
                    .appearance(ButtonAppearance::Accent)
                    .compact(),
            ]),
        )
}
