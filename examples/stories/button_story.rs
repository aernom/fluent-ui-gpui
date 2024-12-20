use fluent::{h_flex, v_flex, Button, ButtonAppearance, ButtonShape, Disableable};
use gpui::{black, div, svg, Div, ParentElement, Styled};

pub fn buttons_page() -> Div {
    v_flex()
        .gap_4()
        .child(
            h_flex().gap_2().children([
                Button::new(1)
                    .label("Primary")
                    .appearance(ButtonAppearance::Primary),
                Button::new(2)
                    .label("Outline")
                    .appearance(ButtonAppearance::Outline),
                Button::new(3)
                    .label("Subtle")
                    .appearance(ButtonAppearance::Subtle),
            ]),
        )
        .child(
            h_flex().gap_2().children([
                Button::new(4)
                    .label("Primary Disabled")
                    .leading(svg().path("send.svg"))
                    .appearance(ButtonAppearance::Primary)
                    .disabled(true),
                Button::new(5)
                    .label("Outline Disabled")
                    .appearance(ButtonAppearance::Outline)
                    .disabled(true),
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
                    .appearance(ButtonAppearance::Outline)
                    .shape(ButtonShape::Circular),
                Button::new(9)
                    .label("Square")
                    .appearance(ButtonAppearance::Outline)
                    .shape(ButtonShape::Square),
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
                    v_flex().items_start().child("Buttons can have").child(
                        div()
                            .text_xs()
                            .text_color(black())
                            .child("SECONDARY CONTENT"),
                    ),
                ),
            ]),
        )
}
