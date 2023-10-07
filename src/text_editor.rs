use iced::{
    advanced::{
        layout, mouse, renderer,
        widget::{self, tree},
        Clipboard, Layout, Shell, Widget,
    },
    event::Status,
    keyboard::{Event as KeyEvent, KeyCode},
    Color, Element, Event, Length, Padding, Size,
};

mod backend;
use backend::Backend;

struct Appearance {
    background_color: Option<Color>,
    text_color: Color,
}

trait Stylesheet {
    fn appearance(&self) -> Appearance;
}

impl Stylesheet for iced::Theme {
    fn appearance(&self) -> Appearance {
        match *self {
            iced::Theme::Light => Appearance {
                background_color: Some(Color::WHITE),
                text_color: Color::BLACK,
            },
            iced::Theme::Dark => Appearance {
                background_color: Some(Color::BLACK),
                text_color: Color::WHITE,
            },
            iced::Theme::Custom(_) => {
                let palette = self.palette();
                Appearance {
                    background_color: Some(palette.background),
                    text_color: palette.text,
                }
            }
        }
    }
}

pub struct State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn text_editor() -> TextEditor {
    TextEditor::new()
}

pub struct TextEditor {
    backend: Backend,
    padding: Padding,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            backend: Backend::default(),
            padding: Padding::new(0.0),
        }
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for TextEditor
where
    Renderer: renderer::Renderer,
    Renderer::Theme: Stylesheet,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new())
    }

    fn width(&self) -> iced::Length {
        Length::Fill
    }

    fn height(&self) -> iced::Length {
        Length::Fill
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(Length::Fill).height(Length::Fill);

        layout::Node::new(limits.resolve(Size::new(limits.max().width, limits.max().height)))
    }

    fn mouse_interaction(
        &self,
        _state: &iced::advanced::widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Text
        } else {
            mouse::Interaction::Idle
        }
    }

    fn draw(
        &self,
        _state: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let appearance = theme.appearance();
        if let Some(bg) = appearance.background_color {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: 1.0.into(),
                    border_width: 1.0,
                    border_color: Color::from_rgb(0.5, 1.0, 0.5),
                },
                bg,
            );
        };
        let mut bounds = layout.bounds();
        let half_p_w = self.padding.horizontal() / 2.0;
        let half_p_h = self.padding.vertical() / 2.0;
        bounds.x += half_p_w;
        bounds.width -= self.padding.horizontal();
        bounds.y += half_p_h;
        bounds.height -= self.padding.vertical();
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border_radius: 1.0.into(),
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            appearance.text_color,
        );
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> Status {
        let state = tree.state.downcast_mut::<State>();

        let mut status = Status::Ignored;
        match event {
            Event::Keyboard(KeyEvent::KeyPressed {
                key_code,
                modifiers,
            }) => {
                println!("Key Pressed: {:?}, {:?}", key_code, modifiers)
            }
            Event::Mouse(event) => {
                println!("Mouse event: {:?}", event)
            }
            _ => {}
        }
        status
    }
}

impl<'a, Message, Renderer> From<TextEditor> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: Stylesheet,
{
    fn from(text_editor: TextEditor) -> Self {
        Self::new(text_editor)
    }
}
