use std::io;

struct Entry {
    datetime_string: String,
    text: String,
}

impl Entry {
    fn new(text: String) -> Self {
        Self {
            datetime_string: String::from("2024-07-14 11:30:00"),
            text,
        }
    }

    fn print_entry(&self) {
        println!("{} - {}", self.datetime_string, self.text);
    }
}

fn main() {
    println!("~ Journal ~");

    loop {
        let mut choice = String::new();

        println!("Main Menu");
        println!("1. Write New Entry.");
        println!("2. Quit.");
        println!("Enter Choice:");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim() {
            "1" => {
                let mut entry_text = String::new();
                println!("Type entry and press enter to submit:");
                io::stdin()
                    .read_line(&mut entry_text)
                    .expect("Failed to read line.");
                let entry_text = entry_text.trim().to_string();
                let entry = Entry::new(entry_text);
                entry.print_entry();
            }
            "2" => {
                println!("Exiting Program...");
                break;
            }
            other => {
                println!("Invalid choice {}. Please try again.", other);
            }
        }
    }
}
