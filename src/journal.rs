struct Entry {
    datetime_string: String,
    text: String,
}

impl Entry {

    fn new(text: &str) -> Self {
        Self {
            datetime_string: String::from("2024-07-14 11:30:00"),
            text: String::from(text),
        }
    }

    fn print_entry(&self) {
        println!("{} - {}", self.datetime_string, self.text);
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

    pub fn print_all_entries(&self) {
        for i in &self.entries {
            i.print_entry();
        }
    }
}
