use std::io;

mod journal;

fn main() {
    println!("~ Journal ~");
    let mut new_journal = journal::Journal::new();

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
                new_journal.new_entry(entry_text.trim());
                let formatted_entries = new_journal.get_formatted_entries();
                for entry in &formatted_entries {
                    println!("{entry}");
                }
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
