use std::time::Duration;
use std::time::Instant;

use iced::advanced::layout;
use iced::advanced::layout::Node;
use iced::advanced::widget::tree;
use iced::advanced::widget::tree::Tag;
use iced::advanced::widget::tree::Tree;
use iced::animation;
use iced::window;
use iced::{Animation, Element, advanced::Widget};
use iced::{Length, Size, Vector, advanced};

pub struct SideBar<'a, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    width: Length,
    height: Length,
    expand: bool,
}

impl<'a, Message, Theme, Renderer> From<SideBar<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: advanced::Renderer + 'a,
{
    fn from(
        container: SideBar<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(container)
    }
}

impl<'a, Message, Theme, Renderer> SideBar<'a, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();
        println!("new 被调用");
        SideBar {
            content: content.into(),
            width: size.width.fluid(),
            height: size.width.fluid(),
            expand: false,
        }
    }

    pub fn expand(mut self, expand: bool) -> Self {
        println!("pressed被调用");
        self.expand = expand;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
}

struct State {
    animation: Animation<bool>,
    width: f32,
}

impl State {
    fn new(width: f32) -> Self {
        Self {
            animation: Animation::new(true).duration(Duration::from_secs(1)),
            width,
        }
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for SideBar<'_, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer,
{
    fn size(&self) -> iced::Size<iced::Length> {
        Size {
            width: self.width.into(),
            height: self.height.into(),
        }
    }

    //limit是父容器的约束
    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        //根据差值布局
        let limt = limits.width(self.width);
        println!("layout被调用:{:?}",limt);
        self.content.as_widget_mut().layout(tree, renderer, &limt)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.content
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn update(
        &mut self,
        tree: &mut advanced::widget::Tree,
        event: &iced::Event,
        layout: layout::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn advanced::Clipboard,
        shell: &mut advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) {
        if let iced::Event::Window(window::Event::RedrawRequested(instant)) = event {
            let state = tree.state.downcast_mut::<State>();
            let animation = &mut state.animation;
            println!("update2:{:?}",instant);

            if animation.value() != self.expand {
                animation.go_mut(self.expand, *instant);
            }

            if animation.is_animating(*instant) {
                let width = state.width;
                let new_width = animation.interpolate(width, 0., *instant);
                self.width=new_width.into();
                shell.request_redraw();
            }
        }

        self.content.as_widget_mut().update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn size_hint(&self) -> Size<Length> {
        self.size()
    }

    fn tag(&self) -> Tag {
        println!("tag被调用");
        Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        println!("Stateg被调用");
        if let Length::Fixed(width) = self.width {
            let state = State::new(width);
            tree::State::Some(Box::new(state))
        } else {
            tree::State::None
        }
    }

    fn children(&self) -> Vec<Tree> {
        println!("children被调用");
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.content.as_widget().diff(tree);
    }

    fn operate(
        &mut self,
        tree: &mut advanced::widget::Tree,
        layout: layout::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn advanced::widget::Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(tree, layout, renderer, operation);
    }

    fn mouse_interaction(
        &self,
        tree: &advanced::widget::Tree,
        layout: layout::Layout<'_>,
        cursor: advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> advanced::mouse::Interaction {
        self.content
            .as_widget()
            .mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut advanced::widget::Tree,
        layout: layout::Layout<'a>,
        renderer: &Renderer,
        viewport: &iced::Rectangle,
        translation: Vector,
    ) -> Option<advanced::overlay::Element<'a, Message, Theme, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(tree, layout, renderer, viewport, translation)
    }
}
