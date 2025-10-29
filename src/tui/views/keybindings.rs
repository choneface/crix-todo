use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Flex, Layout, Rect};
use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn render(f: &mut Frame) {
    let popup = popup_area(f.size(), 60, 50);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(0)])
        .split(popup);

    render_keybindings(f, chunks[0]);
}

fn render_keybindings(f: &mut Frame, chunk: Rect) {
    let keybindings = vec![
        ListItem::new("Normal Mode"),
        ListItem::new("  ╭─ [j/k] Move"),
        ListItem::new("  ├─ [Space] Expand"),
        ListItem::new("  ├─ [i] Edit"),
        ListItem::new("  ├─ [⌫] Delete"),
        ListItem::new("  ├─ [p/l] Toggle Priority"),
        ListItem::new("  ├─ [b] Split Todo"),
        ListItem::new("  ╰─ [Esc] Quit"),
        ListItem::new("Edit Mode"),
        ListItem::new("  ╭─ [Char/Space] Input Char"),
        ListItem::new("  ├─ [←/→] Move cursor"),
        ListItem::new("  ├─ [⌫] Backspace"),
        ListItem::new("  ╰─ [Esc] Return to Normal Mode"),
    ];
    let list =
        List::new(keybindings).block(Block::default().title("Keybindings").borders(Borders::ALL));
    f.render_widget(list, chunk);
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
