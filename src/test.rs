use gpui_kit::{
    IntoElement, ParentElement, Styled, component::TitleBar, div, px,
    relative,
};

pub fn test() -> impl IntoElement {
    div().flex().child(
        div().w(relative(1.)).child(
            TitleBar::new()
                .bg(gpui_kit::blue())
                .flex()
                .child(
                    div()
                        .flex()
                        .h(px(400.))
                        .w(px(40.))
                        .bg(gpui_kit::yellow()),
                )
                .child(div().flex_grow(1.))
                .child(div().size(px(40.)).bg(gpui_kit::red())),
        ),
    )
}
