use std::fs;
use std::error::Error;
use std::io::Write;

pub struct Config {
    file_path: String,
    text: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let file_path = args[1].clone();
        let text = args[2].clone();

        Ok(Config { file_path, text })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let datetime_string = String::from("2024-07-14 11:30:00");
    println!("Writing entry {} to file {}", config.text, config.file_path);
    
    let entry = format!("\n{} - {}", datetime_string, config.text);
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&config.file_path)?;
    
    writeln!(file, "{}", entry)?;

    let contents = fs::read_to_string(config.file_path)?;

    println!("Current entries\n{contents}");

    Ok(())
}