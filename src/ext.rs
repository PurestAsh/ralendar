use gpui_kit::{
    App, Entity, IntoElement, ParentElement, RenderOnce, Styled,
    Window,
    base::ElementExt,
    component::date_picker::{DatePicker, DatePickerState},
    div, px,
};
#[derive(IntoElement)]
pub struct DatePickerAtBaseRem(pub Entity<DatePickerState>);

impl RenderOnce for DatePickerAtBaseRem {
    fn render(
        self,
        window: &mut Window,
        _: &mut App,
    ) -> impl IntoElement {
        let previous_rem_size = window.rem_size();
        window.set_rem_size(px(16.));

        div()
            .size_full()
            .on_prepaint(move |_, window, _| {
                window.set_rem_size(previous_rem_size)
            })
            .child(
                DatePicker::new(&self.0)
                    .appearance(true)
                    .placeholder("开始时间"),
            )
    }
}
