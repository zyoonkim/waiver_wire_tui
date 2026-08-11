use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub enum InputResult {
    Continue,
    Quit,
}

pub fn handle_key(app: &mut App, key: KeyEvent) -> InputResult {
    match key.code {
        KeyCode::Esc => InputResult::Quit,

        KeyCode::Tab => {
            app.next_widget();
            InputResult::Continue
        }

        KeyCode::BackTab => {
            app.previous_widget();
            InputResult::Continue
        }

        KeyCode::Up => {
            app.previous_result();
            InputResult::Continue
        }

        KeyCode::Down => {
            app.next_result();
            InputResult::Continue
        }

        KeyCode::Backspace => {
            app.remove_search_char();
            InputResult::Continue
        }

        KeyCode::Char(c) => {
            app.add_search_char(c);
            InputResult::Continue
        }

        _ => InputResult::Continue,
    }
}
