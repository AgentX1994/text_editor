pub enum Action {
    Insert(char),
    Delete,
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,
    Escape,
    Enter,
    Backspace,
}

#[derive(Default, Debug)]
pub struct Backend {
    content: String,
}

impl Backend {
    pub fn action(&mut self, action: Action) {
        match action {
            Action::Insert(c) => {
                if c.is_control() && !['\t', '\n', '\u{92}'].contains(&c) {
                    // Control character
                } else if c == '\n' {
                    self.action(Action::Enter);
                } else {
                    self.content.push(c);
                }
            }
            Action::Delete => todo!(),
            Action::Up => todo!(),
            Action::Down => todo!(),
            Action::Left => todo!(),
            Action::Right => todo!(),
            Action::Home => todo!(),
            Action::End => todo!(),
            Action::PageUp => todo!(),
            Action::PageDown => todo!(),
            Action::Escape => todo!(),
            Action::Enter => self.content.push('\n'),
            Action::Backspace => {
                let c = self.content.pop();
                println!("Backspace performed! Removed character: {c:?}");
            }
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
