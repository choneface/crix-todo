use crate::tui::app::App;
use crate::tui::app::InputMode::Editing;
use crate::tui::views::todo_list::Row;

pub struct TodoListViewModel<'a> {
    pub rows: Vec<Row<'a>>,
    pub selected_index: Option<usize>,
    pub cursor_position: (u16, u16),
}

impl<'a> TodoListViewModel<'a> {
    pub fn from_app(app: &'a mut App) -> Self {
        let mut rows = Vec::new();
        let mut last_priority: Option<u8> = None;
        let mut selected_index = None;

        for &i in &app.visual_order {
            let todo = &app.todos[i];
            let priority = todo.priority.unwrap_or(99);

            if Some(priority) != last_priority {
                rows.push(Row::Header(match priority {
                    99 => "Priority None".to_string(),
                    p => format!("Priority {}", p),
                }));
                last_priority = Some(priority);
            }

            let is_expanded = app.expanded == Some(i);
            if Some(i) == app.visual_order.get(app.selected).copied() {
                selected_index = Some(rows.len());
            }

            rows.push(Row::Todo {
                item: todo,
                is_expanded,
            });
        }

        let mut x = 0;
        let mut y = 0;
        if app.mode == Editing {
            let buf = app
                .edit_buffer
                .as_mut()
                .expect("editing - edit buffer was null");
            x = 4 + (buf.current_field_mut().cursor as u16);
            y = selected_index.unwrap_or_default() as u16 + 1;
            if app.expanded.is_some() {
                x += 10;
                y += 1;
            }
        }

        Self {
            rows,
            selected_index,
            cursor_position: (x, y),
        }
    }

    pub fn get_cursor_position(&self) -> Option<(u16, u16)> {
        if self.cursor_position != (0, 0) {
            Some(self.cursor_position)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::TodoItem;
    use crate::tui::app::{App, InputMode};
    use crate::tui::state::edit_buffer::EditBuffer;

    fn make_todo(description: &str, priority: Option<u8>, expanded: bool) -> (TodoItem, bool) {
        (
            TodoItem {
                description: description.to_string(),
                priority,
                notes: String::new(),
            },
            expanded,
        )
    }

    fn make_basic_two_todos() -> Vec<TodoItem> {
        vec![
            make_todo("first", Some(1), false).0,
            make_todo("second", Some(1), false).0,
        ]
    }

    fn make_app_with_cursor(
        mut todos: Vec<TodoItem>,
        selected: usize,
        expanded: Option<usize>,
        cursor: u16,
        mode: InputMode,
    ) -> App {
        let is_editing = matches!(mode, InputMode::Editing);

        let mut app = App {
            todos: vec![],
            visual_order: vec![],
            selected,
            expanded,
            mode,
            edit_buffer: None,
        };

        // keep indexes stable for the test
        app.todos = todos.drain(..).collect();
        app.visual_order = (0..app.todos.len()).collect();

        if is_editing {
            // seed the edit buffer based on the currently selected todo
            let idx = app.visual_order[app.selected];
            let mut eb = EditBuffer::new(&app.todos[idx]);
            eb.current_field_mut().cursor = cursor as usize;
            app.edit_buffer = Some(eb);
        }

        app
    }

    #[test]
    fn cursor_position_in_normal_mode_is_none() {
        let todos = make_basic_two_todos();
        let mut app = make_app_with_cursor(
            todos,
            /*selected*/ 0,
            /*expanded*/ None,
            /*cursor*/ 5,
            InputMode::Normal,
        );

        let vm = TodoListViewModel::from_app(&mut app);
        assert_eq!(
            vm.get_cursor_position(),
            None,
            "normal mode should not expose a cursor"
        );
        assert_eq!(vm.cursor_position, (0, 0));
    }

    #[test]
    fn cursor_position_editing_not_expanded_second_item() {
        // Two todos in the same priority group → rows = [Header, Todo0, Todo1]
        // selected = 1 → selected_index should be 2
        // x = 4 + cursor; y = selected_index + 1
        let todos = make_basic_two_todos();
        let mut app = make_app_with_cursor(
            todos,
            /*selected*/ 1,
            /*expanded*/ None,
            /*cursor*/ 7,
            InputMode::Editing,
        );

        let vm = TodoListViewModel::from_app(&mut app);

        // header (0), first todo (1), second todo (2) → y = 2 + 1 = 3
        let expected = (4 + 7, 3);
        assert_eq!(vm.get_cursor_position(), Some(expected));
        assert_eq!(vm.cursor_position, expected);
        assert_eq!(vm.selected_index, Some(2));
    }

    #[test]
    fn cursor_position_editing_expanded_applies_offsets() {
        // Same as above but with expanded set → +10 to x and +1 to y
        let todos = make_basic_two_todos();
        let mut app = make_app_with_cursor(
            todos,
            /*selected*/ 1,
            /*expanded*/ Some(1),
            /*cursor*/ 3,
            InputMode::Editing,
        );

        let vm = TodoListViewModel::from_app(&mut app);

        // baseline without expansion would be (4 + 3, 3)
        // with expansion: x += 10 → 17; y += 1 → 4
        let expected = (17, 4);
        assert_eq!(vm.get_cursor_position(), Some(expected));
        assert_eq!(vm.cursor_position, expected);
        assert_eq!(vm.selected_index, Some(2));
    }

    #[test]
    fn cursor_position_editing_first_item_correct_y() {
        // Select the first todo: rows = [Header, Todo0, Todo1], selected_index = 1
        // y = 1 + 1 = 2 (not expanded)
        let todos = make_basic_two_todos();
        let mut app = make_app_with_cursor(
            todos,
            /*selected*/ 0,
            /*expanded*/ None,
            /*cursor*/ 0,
            InputMode::Editing,
        );

        let vm = TodoListViewModel::from_app(&mut app);

        let expected = (4 + 0, 2);
        assert_eq!(vm.get_cursor_position(), Some(expected));
        assert_eq!(vm.cursor_position, expected);
        assert_eq!(vm.selected_index, Some(1));
    }

    #[test]
    fn cursor_position_handles_mixed_priorities_with_headers() {
        // Mixed priorities → more headers change selected_index math.
        // visual_order: [0(p=1), 1(p=2), 2(p=None)]
        // rows: [H1, T0, H2, T1, HNone, T2]
        // select index 1 (the p=2 item) → selected_index = 3
        let todos = vec![
            make_todo("p1", Some(1), false).0,
            make_todo("p2", Some(2), false).0,
            make_todo("none", None, false).0,
        ];
        let mut app = make_app_with_cursor(
            todos,
            /*selected*/ 1,
            /*expanded*/ None,
            /*cursor*/ 5,
            InputMode::Editing,
        );

        let vm = TodoListViewModel::from_app(&mut app);

        // expected y = 3 + 1 = 4; x = 4 + 5 = 9
        let expected = (9, 4);
        assert_eq!(vm.get_cursor_position(), Some(expected));
        assert_eq!(vm.cursor_position, expected);
        assert_eq!(vm.selected_index, Some(3));
    }

    #[test]
    fn builds_rows_with_headers_and_selection_correctly() {
        let todos = vec![
            make_todo("first", Some(1), false),
            make_todo("second", Some(1), true),
            make_todo("third", Some(2), false),
            make_todo("no-priority", None, false),
        ]
        .into_iter()
        .map(|(todo, _)| todo)
        .collect::<Vec<_>>();

        let app = &mut App {
            todos,
            visual_order: vec![0, 1, 2, 3],
            selected: 1, // select the second tod0
            expanded: Some(1),
            mode: InputMode::Normal,
            edit_buffer: None,
        };

        let vm = TodoListViewModel::from_app(app);

        let headers = vm
            .rows
            .iter()
            .filter_map(|row| {
                if let Row::Header(h) = row {
                    Some(h.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        assert_eq!(headers, vec!["Priority 1", "Priority 2", "Priority None"]);
        assert_eq!(vm.rows.len(), 7); // 3 headers and 4 todos
        assert_eq!(vm.selected_index, Some(2)); // header, 1st tod0, second tod0
    }

    #[test]
    fn marks_expanded_flag_correctly() {
        let todos = vec![
            make_todo("one", Some(1), false),
            make_todo("two", Some(2), true),
        ]
        .into_iter()
        .map(|(todo, _)| todo)
        .collect::<Vec<_>>();

        let app = &mut App {
            todos,
            visual_order: vec![0, 1],
            selected: 0,
            expanded: Some(1),
            mode: InputMode::Normal,
            edit_buffer: None,
        };

        let vm = TodoListViewModel::from_app(app);

        // get a list of is_expanded
        let flags = vm
            .rows
            .iter()
            .filter_map(|row| {
                if let Row::Todo { is_expanded, .. } = row {
                    Some(*is_expanded)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // only the second one should be expanded
        assert_eq!(flags, vec![false, true]);
    }

    #[test]
    fn handles_empty_state() {
        let app = &mut App {
            todos: vec![],
            visual_order: vec![],
            selected: 0,
            expanded: None,
            mode: InputMode::Normal,
            edit_buffer: None,
        };

        let vm = TodoListViewModel::from_app(app);

        assert!(vm.rows.is_empty());
        assert_eq!(vm.selected_index, None);
    }
}
