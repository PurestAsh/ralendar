use gpui_kit::{
    App, Entity, IntoElement, ParentElement, RenderOnce, Styled, Window, px,
    base::ElementExt as _,
    component::{
        button::Button,
        carousel::{
            Carousel, CarouselContent, CarouselItem, CarouselState,
        },
        date_picker::{DatePicker, DatePickerState},
    },
    div,
};

use crate::state::State;

/// Renders the date picker with the rem size its calendar layout assumes,
/// then restores the window's rem size before the rest of the frame is painted.
///
/// gpui-component lays the calendar out as 7 × 2rem day cells inside a
/// fixed 224px-wide grid, which only fits when 1rem is 16px. The Ayu Dark
/// theme sets the font size to 18px, so a day cell becomes 36px and the
/// seven columns (252px) overflow the 224px grid, pushing the last column
/// out of the popup. Use the 16px rem size for this subtree so the cells
/// fit again.
#[derive(IntoElement)]
struct DatePickerAtBaseRem(Entity<DatePickerState>);

impl RenderOnce for DatePickerAtBaseRem {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let previous_rem_size = window.rem_size();
        window.set_rem_size(px(16.));

        div()
            .size_full()
            .on_prepaint(move |_, window, _| window.set_rem_size(previous_rem_size))
            .child(DatePicker::new(&self.0).appearance(true))
    }
}

pub fn test(state: &mut State) -> impl IntoElement {
    DatePickerAtBaseRem(state.start_time_state().clone())
}
