use crate::message::SideBarMsg;
use crate::{message::Message, state::ApplicationState};
use iced::widget::{button, container};
use iced::{Element, Length};

pub fn view<'a>(state: &ApplicationState) -> Element<'a, Message> {
    container(button("press").on_press(Message::SideBarMsg(SideBarMsg::Toggle)))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
