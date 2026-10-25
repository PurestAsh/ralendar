use gpui_kit::{
    Entity, IntoElement, ParentElement, Styled,
    component::{
        button::Button,
        carousel::{
            Carousel, CarouselContent, CarouselItem, CarouselState,
        },
    },
    div,
};

use crate::state::State;

pub fn test(state: &mut State) -> impl IntoElement {
    let carousel = state.carousel_state().clone();
    let previous_carousel = carousel.clone();
    let next_carousel = carousel.clone();

    div()
        .size_full()
        .flex()
        .flex_col()
        .child(
            // 左右切换按钮
            div()
                .flex()
                .items_center()
                .justify_center()
                .gap_2()
                .p_2()
                .child(
                    Button::new("carousel-previous")
                        .label("◀ 上一月")
                        .on_click(move |_, _, cx| {
                            previous_carousel.update(cx, |state, cx| {
                                state.select_previous(cx);
                            });
                        }),
                )
                .child(
                    Button::new("carousel-next")
                        .label("下一月 ▶")
                        .on_click(move |_, _, cx| {
                            next_carousel.update(cx, |state, cx| {
                                state.select_next(cx);
                            });
                        }),
                ),
        )
        .child(
            div().flex_1().w_full().child(
                Carousel::new("month-carousel", &carousel).child(
                    CarouselContent::new(&carousel)
                        .child(my_content(0, &carousel))
                        .child(my_content(1, &carousel))
                        .child(my_content(2, &carousel)),
                ),
            ),
        )
}

pub fn my_content(
    i: usize,
    state: &Entity<CarouselState>,
) -> impl IntoElement {
    // 每个 item 的 id 和 index 必须固定且唯一，不能跟着选中状态变化，
    // 否则每次切换都会重新挂载整个 item，看起来就像页面刷新。
    let background = match i {
        0 => gpui_kit::red(),
        1 => gpui_kit::blue(),
        _ => gpui_kit::yellow(),
    };

    CarouselItem::new(format!("project-{i}"), i, state)
        .child(format!(" 当前是第{}页", i))
        .bg(background)
}
