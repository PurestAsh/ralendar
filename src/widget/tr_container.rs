use iced::Rectangle;
use iced::advanced::layout;
use iced::advanced::layout::Node;
use iced::advanced::widget::tree;
use iced::advanced::widget::tree::Tag;
use iced::advanced::widget::tree::Tree;
use iced::widget::container::Catalog as ContainerCatalog;
use iced::{Element, advanced::Widget};
use iced::{Length, Size, Vector, advanced};

pub struct TrContainer<'a, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    width: f32,
}

impl<'a, Message, Theme, Renderer> From<TrContainer<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: ContainerCatalog + 'a,
    Renderer: advanced::Renderer + 'a,
{
    fn from(
        container: TrContainer<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(container)
    }
}

impl<'a, Message, Theme, Renderer> TrContainer<'a, Message, Theme, Renderer>
where
    Theme: ContainerCatalog,
    Renderer: advanced::Renderer,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        TrContainer {
            content: content.into(),
            width: 0.,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TrContainer<'_, Message, Theme, Renderer>
where
    Theme: ContainerCatalog,
    Renderer: advanced::Renderer,
{
    fn size(&self) -> iced::Size<iced::Length> {
        self.content.as_widget().size()
    }

    //limit是父容器的约束
    fn layout(
        &mut self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> Node {
        self.content.as_widget_mut().layout(tree, renderer, limits)
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
        renderer.with_translation(Vector::new(self.width - 250., 0.), |renderer| {
            self.content
                .as_widget()
                .draw(tree, renderer, theme, style, layout, cursor, viewport);
        });
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
        self.content.as_widget_mut().update(
            tree, event, layout, cursor, renderer, clipboard, shell, viewport,
        );
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn tag(&self) -> Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<Tree> {
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
