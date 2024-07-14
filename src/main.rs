struct Entry {
    datetime_string: String,
    text: String,
}

impl Entry {
    fn print_entry(&self) {
        println!("{} - {}", self.datetime_string, self.text);
    }
}

fn main() {
    println!("~ Journal ~");

    let entry = Entry {
        datetime_string: String::from("2024-07-14 11:30:00"),
        text: String::from("This is my first entry"),
    };

    entry.print_entry();
}
