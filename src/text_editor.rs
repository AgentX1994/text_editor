use std::sync::Mutex;

use iced::advanced::layout::Node;
use iced::widget::text::{draw as draw_text, Appearance as TextAppearance, LineHeight, Shaping};
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
use iced::{alignment, Vector};

pub mod backend;
use backend::Backend;

use self::backend::Action;

struct Appearance {
    background_color: Option<Color>,
    border_color: Option<Color>,
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
                border_color: Some(Color::from_rgb(0.75, 1.0, 0.75)),
                text_color: Color::BLACK,
            },
            iced::Theme::Dark => Appearance {
                background_color: Some(Color::BLACK),
                border_color: Some(Color::from_rgb(0.25, 0.75, 0.25)),
                text_color: Color::WHITE,
            },
            iced::Theme::Custom(_) => {
                let palette = self.palette();
                Appearance {
                    background_color: Some(palette.background),
                    border_color: Some(palette.primary),
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

pub fn text_editor(backend: &Mutex<Backend>) -> TextEditor {
    TextEditor::new(backend)
}

pub struct TextEditor<'a> {
    backend: &'a Mutex<Backend>,
    padding: Padding,
}

impl<'a> TextEditor<'a> {
    pub fn new(backend: &'a Mutex<Backend>) -> Self {
        println!("Creating Text Editor!");
        Self {
            backend,
            padding: Padding::new(0.0),
        }
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for TextEditor<'a>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer,
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
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let appearance = theme.appearance();
        if let Some(bg) = appearance.background_color {
            let border_color = if let Some(c) = appearance.border_color {
                c
            } else {
                Color::TRANSPARENT
            };
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: 1.0.into(),
                    border_width: 1.0,
                    border_color,
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
        // TODO reimplement my own text handling
        let fake_node = Node::new(Size {
            width: bounds.width,
            height: bounds.height,
        });
        let text_layout = Layout::with_offset(Vector::new(bounds.x, bounds.y), &fake_node);
        let backend = self.backend.lock().expect("Poisoned");
        println!("Text to render: {}", backend.content());
        draw_text(
            renderer,
            style,
            text_layout,
            backend.content(),
            None,
            LineHeight::default(),
            None,
            TextAppearance {
                color: Some(appearance.text_color),
            },
            alignment::Horizontal::Left,
            alignment::Vertical::Top,
            Shaping::Advanced,
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
        let mut backend = self.backend.lock().expect("Poisoned");

        let mut status = Status::Ignored;
        match event {
            Event::Keyboard(KeyEvent::KeyPressed {
                key_code,
                modifiers,
            }) => match key_code {
                KeyCode::Left => {
                    backend.action(Action::Left);
                    status = Status::Captured;
                }
                KeyCode::Right => {
                    backend.action(Action::Right);
                    status = Status::Captured;
                }
                KeyCode::Up => {
                    backend.action(Action::Up);
                    status = Status::Captured;
                }
                KeyCode::Down => {
                    backend.action(Action::Down);
                    status = Status::Captured;
                }
                KeyCode::PageUp => {
                    backend.action(Action::PageUp);
                    status = Status::Captured;
                }
                KeyCode::PageDown => {
                    backend.action(Action::PageDown);
                    status = Status::Captured;
                }
                KeyCode::Home => {
                    backend.action(Action::Home);
                    status = Status::Captured;
                }
                KeyCode::End => {
                    backend.action(Action::End);
                    status = Status::Captured;
                }
                KeyCode::Escape => {
                    backend.action(Action::Escape);
                    status = Status::Captured;
                }
                KeyCode::Enter => {
                    backend.action(Action::Enter);
                    status = Status::Captured;
                }
                KeyCode::Backspace => {
                    println!("Backspace pressed!");
                    backend.action(Action::Backspace);
                    status = Status::Captured;
                }
                KeyCode::Delete => {
                    backend.action(Action::Delete);
                    status = Status::Captured;
                }
                _ => {}
            },
            Event::Keyboard(KeyEvent::CharacterReceived(character)) => {
                println!("Char received: {character}");
                backend.action(Action::Insert(character));
                status = Status::Captured;
            }
            Event::Mouse(_event) => {
                // println!("Mouse event: {:?}", event)
            }
            _ => {}
        }
        status
    }
}

impl<'a, Message, Renderer> From<TextEditor<'a>> for Element<'a, Message, Renderer>
where
    Renderer: renderer::Renderer + iced::advanced::text::Renderer,
    Renderer::Theme: Stylesheet,
{
    fn from(text_editor: TextEditor<'a>) -> Self {
        Self::new(text_editor)
    }
}
