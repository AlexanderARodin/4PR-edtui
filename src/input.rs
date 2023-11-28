pub mod key;
pub mod register;

use crate::{buffer::mode::Mode, EditorBuffer};

use self::{
    key::Key,
    register::{Register, RegisterKey, RegisterVal},
};

#[derive(Clone, Debug)]
pub struct Input {
    register: Register,
}

impl Default for Input {
    fn default() -> Self {
        let mut register = Register::new();

        // Go into normal mode
        register.insert(
            RegisterKey::i(vec![Key::Esc]),
            RegisterVal(EditorBuffer::normal_mode),
        );
        register.insert(
            RegisterKey::v(vec![Key::Esc]),
            RegisterVal(EditorBuffer::normal_mode),
        );

        // Go into insert mode
        register.insert(
            RegisterKey::n(vec![Key::Char('i')]),
            RegisterVal(EditorBuffer::insert_mode),
        );

        // Go into visual mode
        register.insert(
            RegisterKey::n(vec![Key::Char('v')]),
            RegisterVal(EditorBuffer::visual_mode),
        );

        // Go into insert mode and move one char forward
        register.insert(
            RegisterKey::n(vec![Key::Char('a')]),
            RegisterVal(EditorBuffer::append_mode),
        );

        // Create a new line below and switch to insert mode
        register.insert(
            RegisterKey::n(vec![Key::Char('o')]),
            RegisterVal(EditorBuffer::new_line_below_and_insert_mode),
        );

        // Create a new line above and switch to insert mode
        register.insert(
            RegisterKey::n(vec![Key::Char('O')]),
            RegisterVal(EditorBuffer::new_line_above_and_insert_mode),
        );

        // Move cursor left
        register.insert(
            RegisterKey::n(vec![Key::Char('h')]),
            RegisterVal(EditorBuffer::move_cursor_left),
        );
        register.insert(
            RegisterKey::v(vec![Key::Char('h')]),
            RegisterVal(EditorBuffer::move_cursor_left),
        );
        register.insert(
            RegisterKey::i(vec![Key::Left]),
            RegisterVal(EditorBuffer::move_cursor_left),
        );

        // Move cursor right
        register.insert(
            RegisterKey::n(vec![Key::Char('l')]),
            RegisterVal(EditorBuffer::move_cursor_right),
        );
        register.insert(
            RegisterKey::v(vec![Key::Char('l')]),
            RegisterVal(EditorBuffer::move_cursor_right),
        );
        register.insert(
            RegisterKey::i(vec![Key::Right]),
            RegisterVal(EditorBuffer::move_cursor_right),
        );

        // Move cursor up
        register.insert(
            RegisterKey::n(vec![Key::Char('k')]),
            RegisterVal(EditorBuffer::move_cursor_up),
        );
        register.insert(
            RegisterKey::v(vec![Key::Char('k')]),
            RegisterVal(EditorBuffer::move_cursor_up),
        );
        register.insert(
            RegisterKey::i(vec![Key::Up]),
            RegisterVal(EditorBuffer::move_cursor_up),
        );

        // Move cursor down
        register.insert(
            RegisterKey::n(vec![Key::Char('j')]),
            RegisterVal(EditorBuffer::move_cursor_down),
        );
        register.insert(
            RegisterKey::v(vec![Key::Char('j')]),
            RegisterVal(EditorBuffer::move_cursor_down),
        );
        register.insert(
            RegisterKey::i(vec![Key::Down]),
            RegisterVal(EditorBuffer::move_cursor_down),
        );

        // Insert new line
        register.insert(
            RegisterKey::i(vec![Key::Enter]),
            RegisterVal(EditorBuffer::insert_newline),
        );

        // Remove the current character
        register.insert(
            RegisterKey::n(vec![Key::Char('x')]),
            RegisterVal(EditorBuffer::remove_char),
        );

        // Delete the previous character
        register.insert(
            RegisterKey::i(vec![Key::Backspace]),
            RegisterVal(EditorBuffer::delete_char),
        );

        // Delete the current line
        register.insert(
            RegisterKey::n(vec![Key::Char('d'), Key::Char('d')]),
            RegisterVal(EditorBuffer::delete_line),
        );

        // Delete the current selection
        register.insert(
            RegisterKey::v(vec![Key::Char('d')]),
            RegisterVal(|b: &mut EditorBuffer| {
                b.delete_selection();
                b.normal_mode();
            }),
        );

        // Select inner word between delimiters
        register.insert(
            RegisterKey::n(vec![Key::Char('c'), Key::Char('w')]),
            RegisterVal(|b: &mut EditorBuffer| {
                b.select_between_delimiters(&['"']);
                b.mode = Mode::Visual;
            }),
        );

        // Undo
        register.insert(
            RegisterKey::n(vec![Key::Char('u')]),
            RegisterVal(EditorBuffer::undo),
        );

        // Redo
        register.insert(
            RegisterKey::n(vec![Key::Char('r')]),
            RegisterVal(EditorBuffer::redo),
        );

        Self { register }
    }
}

impl Input {
    pub fn on_key<T>(&mut self, key: T, buffer: &mut EditorBuffer)
    where
        T: Into<Key> + Copy,
    {
        let mode = buffer.mode;

        match key.into() {
            // Always insert characters in insert mode
            Key::Char(c) if mode == Mode::Insert => buffer.insert_char(c),
            // Else lookup an action from the register
            _ => {
                if let Some(cb) = self.register.get(key.into(), mode) {
                    cb(buffer);
                }
            }
        }
    }
}
