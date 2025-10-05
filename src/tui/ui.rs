use crate::tui::app::App;
use crate::tui::views::todo_list;
use ratatui::Frame;

pub fn render(f: &mut Frame, app: &mut App) {
    todo_list::render(f, app);
}
