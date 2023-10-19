use std::sync::Mutex;
use std::time::{Duration, Instant};

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
    window::Event as WindowEvent,
    Color, Element, Event, Length, Padding, Size,
};
use iced::{alignment, window, Pixels, Rectangle, Vector};

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

pub struct State {
    is_focused: bool,
    focus_start: Instant,
    now: Instant,
}

impl Default for State {
    fn default() -> Self {
        Self {
            is_focused: true,
            focus_start: Instant::now(),
            now: Instant::now(),
        }
    }
}

pub fn text_editor(backend: &Mutex<Backend>) -> TextEditor {
    TextEditor::new(backend)
}

const CURSOR_BLINK_INTERVAL_MILLIS: u128 = 500;

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
        tree::State::new(State::default())
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
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
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
        let content = backend.content();
        let line_height = LineHeight::default();
        draw_text(
            renderer,
            style,
            text_layout,
            &content,
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

        // Draw cursor
        let (cursor_row, cursor_column) = backend.get_cursor_position();
        let text_size = renderer.default_size();
        let height: f32 = line_height.to_absolute(Pixels::from(text_size)).into();
        let y = height * cursor_row as f32;
        let line = content.split('\n').nth(cursor_row).unwrap_or("");
        let x = renderer.measure_width(
            &line[0..cursor_column],
            text_size,
            renderer.default_font(),
            Shaping::Advanced,
        );
        let width = 2.0f32;
        if state.is_focused {
            let is_cursor_visible =
                ((state.now - state.focus_start).as_millis() / CURSOR_BLINK_INTERVAL_MILLIS) % 2
                    == 0;
            if is_cursor_visible {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: bounds.x + x,
                            y: bounds.y + y,
                            width,
                            height,
                        },
                        border_radius: 0.0.into(),
                        border_width: 0.0f32,
                        border_color: Color::TRANSPARENT,
                    },
                    appearance.text_color,
                );
            }
        }
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> Status {
        let state = tree.state.downcast_mut::<State>();
        let mut backend = self.backend.lock().expect("Poisoned");

        let mut status = Status::Ignored;
        match event {
            Event::Keyboard(KeyEvent::KeyPressed {
                key_code,
                modifiers,
            }) => {
                if state.is_focused {
                    match key_code {
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
                    }
                }
            }
            Event::Keyboard(KeyEvent::CharacterReceived(character)) => {
                if state.is_focused {
                    println!("Char received: {character}");
                    backend.action(Action::Insert(character));
                    status = Status::Captured;
                }
            }
            Event::Mouse(_event) => {
                // println!("Mouse event: {:?}", event)
            }
            Event::Window(event) => {
                println!("Window Event: {:?}", event);
                match event {
                    WindowEvent::Resized {
                        width: _width,
                        height: _height,
                    } => {
                        // TODO: resizing
                    }
                    WindowEvent::RedrawRequested(now) => {
                        if state.is_focused {
                            state.now = Instant::now();

                            let millis_until_redraw = CURSOR_BLINK_INTERVAL_MILLIS
                                - (now - state.focus_start).as_millis()
                                    % CURSOR_BLINK_INTERVAL_MILLIS;

                            shell.request_redraw(window::RedrawRequest::At(
                                now + Duration::from_millis(millis_until_redraw as u64),
                            ));
                        }
                    }
                    WindowEvent::CloseRequested => todo!(),
                    WindowEvent::Focused => {
                        state.is_focused = true;
                        state.focus_start = Instant::now();
                        shell.request_redraw(window::RedrawRequest::NextFrame);
                    }
                    WindowEvent::Unfocused => state.is_focused = false,
                    _ => {}
                }
            }
            _ => {}
        }
        if status == Status::Captured && state.is_focused {
            state.focus_start = Instant::now();
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
