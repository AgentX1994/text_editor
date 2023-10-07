use iced::{
    widget::{button, container, row, text, Column},
    Element, Length, Sandbox, Theme,
};
use text_editor::text_editor;

mod text_editor;

pub struct Editor {
    content: String,
    theme: Theme,
    submitted: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    TextInput(String),
    TextSubmit,
    ChangeTheme,
    RemoveInput(usize),
}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self {
            content: String::new(),
            theme: Theme::Dark,
            submitted: vec![],
        }
    }

    fn title(&self) -> String {
        "Text Editor".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::TextInput(s) => self.content = s,
            Message::TextSubmit => {
                self.submitted.push(std::mem::take(&mut self.content));
            }
            Message::ChangeTheme => {
                self.theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                    Theme::Custom(_) => unreachable!(),
                }
            }
            Message::RemoveInput(index) => {
                self.submitted.remove(index);
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let theme_button = button("Change Theme").on_press(Message::ChangeTheme);
        let input = text_editor().padding(10.0);
        let mut col = Column::new();
        col = col.push(theme_button);
        col = col.push(input);
        for (i, s) in self.submitted.iter().enumerate() {
            let s_text = text(s);
            let button = button("X").on_press(Message::RemoveInput(i));
            let row = row!(s_text, button);
            let container = container(row);
            col = col.push(container);
        }
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
