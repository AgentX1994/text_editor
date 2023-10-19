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

#[derive(Debug)]
pub struct Backend {
    content: Vec<String>, // A vector of rows
    cursor_row: usize,
    cursor_column: usize,
}

impl Default for Backend {
    fn default() -> Self {
        Self {
            content: vec![String::new()],
            cursor_row: Default::default(),
            cursor_column: Default::default(),
        }
    }
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
                    self.content
                        .get_mut(self.cursor_row)
                        .expect("Cursor went beyond available rows!")
                        .insert(self.cursor_column, c);
                    self.cursor_column += 1;
                }
            }
            Action::Delete => {
                // TODO: Clean up this
                if self.cursor_column
                    != self
                        .content
                        .get(self.cursor_row)
                        .expect("Cursor went beyond available rows!")
                        .len()
                {
                    let line = self
                        .content
                        .get_mut(self.cursor_row)
                        .expect("Cursor went beyond available rows!");
                    line.remove(self.cursor_column);
                } else if self.cursor_row < self.content.len() {
                    let source_line = self.content.remove(self.cursor_row);
                    self.content
                        .get_mut(self.cursor_row)
                        .expect("Cursor went beyond available rows!")
                        .push_str(&source_line);
                }
            }
            Action::Up => {
                self.cursor_row = self.cursor_row.saturating_sub(1);
                self.cursor_column = self.cursor_column.min(
                    self.content
                        .get(self.cursor_row)
                        .expect("Cursor went beyond available rows!")
                        .len(),
                )
            }
            Action::Down => {
                self.cursor_row = self
                    .cursor_row
                    .saturating_add(1)
                    .min(self.content.len().saturating_sub(1));
                self.cursor_column = self.cursor_column.min(
                    self.content
                        .get(self.cursor_row)
                        .expect("Cursor went beyond available rows!")
                        .len(),
                )
            }
            Action::Left => self.cursor_column = self.cursor_column.saturating_sub(1),
            Action::Right => {
                let line_length = self
                    .content
                    .get(self.cursor_row)
                    .expect("Cursor went beyond available rows!")
                    .len();
                self.cursor_column = self.cursor_column.saturating_add(1);
                if self.cursor_column > line_length {
                    self.cursor_column = 0;
                    self.cursor_row = self
                        .cursor_row
                        .saturating_add(1)
                        .min(self.content.len().saturating_sub(1));
                }
            }
            Action::Home => self.cursor_column = 0,
            Action::End => {
                let text = self
                    .content
                    .get(self.cursor_row)
                    .expect("Cursor went beyond available rows!");
                self.cursor_column = text.len();
            }
            Action::PageUp => {
                self.cursor_row = 0;
                self.cursor_column = 0
            }
            Action::PageDown => {
                self.cursor_row = self.content.len().saturating_sub(1);
                self.cursor_column = self
                    .content
                    .get(self.cursor_row)
                    .expect("Cursor went beyond available rows!")
                    .len();
            }
            Action::Escape => todo!(),
            Action::Enter => {
                let end_of_line = self
                    .content
                    .get_mut(self.cursor_row)
                    .expect("Cursor went beyond available rows!")
                    .split_off(self.cursor_column);
                self.content
                    .insert(self.cursor_row.saturating_add(1), end_of_line);
                self.cursor_row += 1;
                self.cursor_column = 0;
            }
            Action::Backspace => {
                if self.cursor_column == 0 {
                    if self.cursor_row != 0 {
                        let removed_line = self.content.remove(self.cursor_row);
                        self.cursor_row = self.cursor_row.saturating_sub(1);
                        let destination_line = self
                            .content
                            .get_mut(self.cursor_row)
                            .expect("Cursor went beyond available rows!");
                        self.cursor_column = destination_line.len();
                        destination_line.push_str(&removed_line);
                    }
                } else {
                    let c = self
                        .content
                        .get_mut(self.cursor_row)
                        .expect("Cursor went beyong available rows!")
                        .pop();
                    self.cursor_column = self.cursor_column.saturating_sub(1);
                    println!("Backspace performed! Removed character: {c:?}");
                }
            }
        }
    }

    pub fn content(&self) -> String {
        self.content
            .iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.cursor_row, self.cursor_column)
    }
}
