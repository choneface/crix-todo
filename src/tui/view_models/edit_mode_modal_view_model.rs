use crate::tui::app::App;
use crate::tui::state::field_buffer::FieldBuffer;

pub struct Input {
    pub title: String,
    pub value: String,
    pub character_index: usize,
    pub selected: bool,
}

pub struct EditModeModalViewModel {
    pub fields: Vec<Input>,
    pub selected_index: usize,
}

impl EditModeModalViewModel {
    pub fn from_app(app: &App) -> Self {
        let buf = app.edit_buffer.as_ref().expect("missing buffer");
        let to_input = |title: &str, fb: &FieldBuffer, idx: usize| Input {
            title: title.to_string(),
            value: fb.value.clone(),
            character_index: fb.cursor,
            selected: idx == buf.selected_field,
        };

        Self {
            fields: vec![
                to_input("Description", &buf.fields[0], 0),
                to_input("Priority", &buf.fields[1], 1),
                to_input("Notes", &buf.fields[2], 4),
            ],
            selected_index: buf.selected_field,
        }
    }
}
