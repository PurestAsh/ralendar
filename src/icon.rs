use gpui_kit::{self as gpui, ParentElement};

use gpui::{
    IntoElement, PathBuilder, Styled, base::StyledExt,
    canvas, div, px,
};

pub fn mini() -> impl IntoElement {
    div().child(
        canvas(
            |_, _, _| {},
            |mut start, s, w, app| {
                start.origin.y +=
                    px(start.size.height / px(2.));
                let mut end = start.origin;
                end.x += start.size.width;

                let mut path_builder =
                    PathBuilder::stroke(px(2.));
                path_builder.move_to(start.origin);
                path_builder.line_to(end);

                w.paint_path(
                    path_builder.build().unwrap(),
                    gpui::black(),
                );
            },
        )
        .bg(gpui::red())
        .margins(10.)
        .h(px(15.))
        .w(px(15.)),
    )
}
