mod text_manipulation;

struct Entry {
    datetime_string: String,
    text: String,
    piglatin_text: String,
}

impl Entry {

    fn new(text: &str) -> Self {
        Self {
            datetime_string: String::from("2024-07-14 11:30:00"),
            text: String::from(text),
            piglatin_text: text_manipulation::convert_to_piglatin(text),
        }
    }

    fn format_entry(&self) -> String {
        format!("{} - {}\nPiglatin: {}", self.datetime_string, self.text, self.piglatin_text)
    }
}

pub struct Journal {
    name: String,
    entries: Vec<Entry>,
}

impl Journal {

    pub fn new() -> Self {
        Self {
            name: String::from("James' Journal"),
            entries: Vec::new(),
        }
    }

    pub fn new_entry(&mut self, text: &str) {
        let entry = Entry::new(text);
        self.entries.push(entry);
    }

    pub fn get_formatted_entries(&self) -> Vec<String> {
        let mut formatted_entries = Vec::new();
        for entry in &self.entries {
            formatted_entries.push(entry.format_entry());
        }
        formatted_entries
    }
}
