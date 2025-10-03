use crate::storage::TodoItem;
use crate::tui::state::field_buffer::FieldBuffer;

pub struct EditBuffer {
    pub fields: [FieldBuffer; 3], // 0-4: desc, prio, notes
    pub selected_field: usize,
}

impl EditBuffer {
    pub fn new(todo: &TodoItem) -> Self {
        Self {
            fields: [
                FieldBuffer::new(todo.description.clone()),
                FieldBuffer::new(todo.priority.map_or(String::new(), |p| p.to_string())),
                FieldBuffer::new(todo.notes.clone().unwrap_or_default()),
            ],
            selected_field: 0,
        }
    }

    pub fn update_todo(&self, todo: &mut TodoItem) {
        todo.description = self.fields[0].value.clone();

        todo.priority = self.fields[1].value.trim().parse::<u8>().ok();

        todo.notes = match self.fields[2].value.trim() {
            "" => None,
            s => Some(s.to_string()),
        };
    }

    pub fn current_field_mut(&mut self) -> &mut FieldBuffer {
        &mut self.fields[self.selected_field]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::TodoItem;

    fn sample_todo() -> TodoItem {
        TodoItem {
            description: "old desc".into(),
            priority: Some(2),
            notes: Some("old note".into()),
        }
    }

    #[test]
    fn new_populates_fields_and_cursors() {
        let todo = sample_todo();
        let buffer = EditBuffer::new(&todo);

        assert_eq!(buffer.fields[0].value, "old desc");
        assert_eq!(buffer.fields[0].cursor, "old desc".chars().count());
        assert_eq!(buffer.selected_field, 0);
    }

    #[test]
    fn current_field_mut_returns_selected_field() {
        let todo = sample_todo();
        let mut buf = EditBuffer::new(&todo);

        buf.selected_field = 2; // Note field
        buf.current_field_mut().value.push_str("X"); // mutate through helper
        assert_eq!(buf.fields[2].value, "old noteX");
    }

    #[test]
    fn update_todo_writes_back_and_parses_correctly() {
        let mut todo = sample_todo();
        let mut buf = EditBuffer::new(&todo);

        buf.fields[0].value = "new desc".into();
        buf.fields[1].value = "5".into();
        buf.fields[2].value = "new note".into();

        buf.update_todo(&mut todo);

        assert_eq!(todo.description, "new desc");
        assert_eq!(todo.priority, Some(5));
        assert_eq!(todo.notes, Some("new note".into()));
    }

    #[test]
    fn invalid_priority_becomes_none() {
        // Edge-case: invalid priority keeps None rather than panicking
        let mut todo = sample_todo();
        let mut buf = EditBuffer::new(&todo);

        buf.fields[1].value = "not-a-number".into();
        buf.update_todo(&mut todo);

        assert_eq!(todo.priority, None);
    }
}
