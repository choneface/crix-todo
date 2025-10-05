use crate::storage::TodoItem;
use crate::tui::app::App;
use crate::tui::app::InputMode::Editing;
use crate::tui::view_models::todo_view_model::TodoListViewModel;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

pub enum Row<'a> {
    Header(String),
    Todo {
        item: &'a TodoItem,
        is_expanded: bool,
    },
}

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2), // keybindings
            Constraint::Min(0),    // list
        ])
        .split(f.size());

    render_keybindings(f, chunks[0]);
    render_todo_list(f, app, chunks[1]);
    render_cursor(f, app, chunks[1]);
}

fn render_todo_list(f: &mut Frame, app: &App, chunk: Rect) {
    let view_model = TodoListViewModel::from_app(app);

    // FIX: actually call the renderer with (index, row)
    let items: Vec<ListItem> = view_model
        .rows
        .iter()
        .enumerate()
        .map(|(i, row)| render_row(i, row, &view_model.rows))
        .collect();

    let mut state = ListState::default();
    state.select(view_model.selected_index);

    let list = List::new(items)
        .block(Block::default().title("Todos").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(list, chunk, &mut state);
}

fn render_keybindings(f: &mut Frame, rect: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::raw("[j/k] Move    "),
        Span::raw("[⏎] Toggle Done    "),
        Span::raw("[Space] Expand    "),
        Span::raw("[e] Edit    "),
        Span::raw("[⌫] Delete    "),
        Span::raw("[p/l] Toggle Priority    "),
        Span::raw("[b] Split Todo    "),
        Span::raw("[Esc] Quit"),
    ]))
    .block(Block::default());

    f.render_widget(header, rect);
}

fn render_cursor(f: &mut Frame, app: &mut App, area: Rect) {
    if app.mode == Editing {
        if app.mode == Editing {
            let buf = app
                .edit_buffer
                .as_mut()
                .expect("editing - edit buffer was null");
            if app.expanded.is_none() {
                let x = area.x + 4 + (buf.current_field_mut().cursor as u16);
                let y = area.y + 2 + (app.selected as u16);
                f.set_cursor(x, y);
            } else {
                let x = area.x + 14 + (buf.current_field_mut().cursor as u16);
                let y = area.y + 3 + (app.selected as u16);
                f.set_cursor(x, y);
            }
        }
    }
}

fn render_row<'a>(idx: usize, row: &Row<'a>, rows: &[Row<'a>]) -> ListItem<'a> {
    match row {
        Row::Header(text) => {
            // Headers are printed plainly (you could add styling here if you like)
            ListItem::new(text.clone())
        }
        Row::Todo { item, is_expanded } => {
            // Determine neighbors and whether we're the first/last todo in the current group.
            let prev = idx.checked_sub(1).and_then(|i| rows.get(i));
            let next = rows.get(idx + 1);

            let prev_is_header = matches!(prev, Some(Row::Header(_))) || prev.is_none();
            let next_is_header = matches!(next, Some(Row::Header(_))) || next.is_none();

            // Choose the connector for this todo line
            // First todo under a header -> "╭─", last -> "╰─", middle -> "├─"
            let todo_connector = if prev_is_header && next_is_header {
                // single item group (both first and last)
                "╰─"
            } else if prev_is_header {
                "╭─"
            } else if next_is_header {
                "╰─"
            } else {
                "├─"
            };

            // Left gutter that continues vertical trunk if there are following siblings in the same group
            // If we're NOT last, keep a vertical line "│  " so child rows look connected.
            let trunk_gutter = if next_is_header { "   " } else { "│  " };

            let mut lines = Vec::new();

            // The todo line
            lines.push(format!("{} {}", todo_connector, item.description));

            // Optional notes, rendered as a curved child branch.
            if *is_expanded {
                if let Some(notes) = &item.notes {
                    // Child branch under the todo:
                    // we keep the trunk (│) in the left gutter when more todos follow.
                    // Then draw the curved child "╰─" into the note.
                    lines.push(format!("{}╰─ Notes: {}", trunk_gutter, notes));
                }
            }

            ListItem::new(lines.join("\n"))
        }
    }
}
