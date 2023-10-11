use std::sync::Mutex;

use iced::{
    widget::{button, container, Column},
    Element, Length, Sandbox, Theme,
};
use text_editor::text_editor;

mod text_editor;

pub struct Editor {
    theme: Theme,
    content: Mutex<text_editor::backend::Backend>,
}

#[derive(Clone, Debug)]
pub enum Message {
    ChangeTheme,
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            content: Mutex::new(text_editor::backend::Backend::default()),
        }
    }

    fn title(&self) -> String {
        "Text Editor".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ChangeTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    Theme::Custom(_) => unreachable!(),
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let theme_button = button("Change Theme").on_press(Message::ChangeTheme);
        let input = text_editor(&self.content).padding(10.0);
        let mut col = Column::new();
        col = col.push(theme_button);
        col = col.push(input);
        container(col)
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
