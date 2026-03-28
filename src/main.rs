use crate::component::side_bar;
use crate::{message::Message, state::ApplicationState, ui::main_page};
use iced::{Element, Settings, Task, Theme, application};
use iced::{Subscription, window};

mod component;
mod message;
mod state;
mod style;
mod ui;
mod widget;

fn main() {
    application(boot, update, view)
        .subscription(subscription)
        .settings(Settings {
            id: Some(String::from("ralander")),
            ..Default::default()
        })
        .font(include_bytes!("../assets/font/NotoSansCJKsc-Bold.otf"))
        .font(include_bytes!("../assets/font/NotoSansCJKsc-Regular.otf"))
        .window(window::Settings {
            size: (1000, 700).into(),
            decorations: true,
            min_size: Some((800, 500).into()),
            ..Default::default()
        })
        .theme(theme)
        .run()
        .unwrap();
}

fn theme(state: &ApplicationState) -> Option<Theme> {
    Some(Theme::Light)
}

fn boot() -> (ApplicationState, Task<Message>) {
    (ApplicationState::default(), Task::none())
}

fn view<'a>(state: &'a ApplicationState) -> Element<'a, Message> {
    main_page(state)
}

fn update(state: &mut ApplicationState, msg: Message) -> Task<Message> {
    match msg {
        Message::SideBarMsg(msg) => side_bar::update(state, msg),
        _ => Task::none(),
    }
}

fn subscription(state: &ApplicationState) -> Subscription<Message> {
    window::events().map(|(id, event)| {
        println!("id:{},event:{:?}", id, event);
        Message::WindowEvent
    })
}
