use crate::tui::app::{App, InputMode};
use crate::tui::views::keybindings;
use crate::tui::views::todo_list;
use ratatui::Frame;

pub fn render(f: &mut Frame, app: &mut App) {
    todo_list::render(f, app);
    if app.mode == InputMode::HelpMenu {
        keybindings::render(f)
    }
}
