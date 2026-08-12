use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, CurrentWidget};

pub enum InputResult {
    Continue,
    Quit,
    RemoveFromWishlist,
    AddToWishlist,
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
            if app.current_widget() == CurrentWidget::SearchBox {
                app.remove_search_char();
                InputResult::Continue
            } else {
                InputResult::RemoveFromWishlist
            }
        }

        KeyCode::Enter => {
            if app.current_widget() == CurrentWidget::SearchBox {
                InputResult::AddToWishlist
            } else {
                InputResult::Continue
            }
        }

        KeyCode::Char(c) => {
            app.add_search_char(c);
            InputResult::Continue
        }

        _ => InputResult::Continue,
    }
}
