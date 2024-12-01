use std::fs;
use std::error::Error;
use std::io::Write;

pub struct Config {
    mode: String,
    journal_name: String,
    text: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("not enough arguments")
        } else if args[1] == String::from("new") || args[1] == String::from("display") {
            let mode = args[1].clone();
            let journal_name = args[2].clone();
            let text = String::from("");
            Ok(Config { mode, journal_name, text })
        } else if args[1] == String::from("add") && args.len() < 4 {
            Err("not enough arguments for mode append")
        } else if args[1] == String::from("add") {
            let mode = args[1].clone();
            let journal_name = args[2].clone();
            let text = args[3].clone();
            Ok(Config { mode, journal_name, text })
        } else {
            Err("invalid mode")
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let journal_file_name = format!("{}_journal.txt", config.journal_name);
    if config.mode == String::from("new") {
        return create_journal(&journal_file_name, &config.journal_name);
    } else if config.mode == String::from("display") {
        return display_entries(&journal_file_name);
    } else if config.mode == String::from("add") {
        return add_entry(&journal_file_name, &config.text);
    }
    Ok(())
}

pub fn create_journal(journal_file_name: &String, journal_name: &String) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(journal_file_name)?;
    writeln!(file, "~ {journal_name} ~\n\n")?;
    println!("New journal created : {journal_name}");
    Ok(())
}

pub fn add_entry(journal_file_name: &String, text: &String) -> Result<(), Box<dyn Error>> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(journal_file_name)?;
    let datetime_string = String::from("2024-07-14 11:30:00");
    let entry = format!("{} - {}\n", datetime_string, text);
    writeln!(file, "{}", entry)?;
    println!("New entry added : {entry}");
    Ok(())
}

pub fn display_entries(journal_file_name: &String)  -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(journal_file_name)?;
    println!("{contents}");
    Ok(())
}