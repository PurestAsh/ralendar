use lilt::FloatRepresentable;

#[derive(Debug, Clone)]
pub enum Message {
    WindowEvent,
    SideBarMsg(SideBarMsg),
    ContentMsg(ContentMsg),
    StateInitialFinished,
}

#[derive(Debug, Clone)]
pub enum SideBarMsg {
    Toggle,
    Animation,
}

#[derive(Debug, Clone)]
pub enum ContentMsg {
    Header(DateType),
    Animation,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum DateType {
    #[default]
    Year,
    Month,
    Day,
}

impl FloatRepresentable for DateType {
    fn float_value(&self) -> f32 {
        match self {
            DateType::Year => 0.,
            DateType::Month => 1.,
            DateType::Day => 2.,
        }
    }
}
