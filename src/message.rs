#[derive(Debug, Clone)]
pub enum Message {
    WindowEvent,
    SideBarMsg(SideBarMsg),
}

#[derive(Debug, Clone)]
pub enum SideBarMsg {
    Toggle,
    Animation,
}
