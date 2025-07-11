use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub enum InputEvent {
    Quit,
    Down,
    Up,
    ToggleDone,
    ToggleExpand,
    None,
}

pub fn poll_input(timeout: Duration) -> std::io::Result<InputEvent> {
    if event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            return Ok(match key.code {
                KeyCode::Char('q') => InputEvent::Quit,
                KeyCode::Down => InputEvent::Down,
                KeyCode::Up => InputEvent::Up,
                KeyCode::Enter => InputEvent::ToggleDone,
                KeyCode::Char(' ') => InputEvent::ToggleExpand,
                _ => InputEvent::None,
            });
        }
    }
    Ok(InputEvent::None)
}
