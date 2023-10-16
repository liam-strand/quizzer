use colored::Colorize;
use crossterm::{cursor, execute, terminal};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use text_io::read;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,
    pub levels: Vec<Level>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    name: String,
    prompt: String,
    key: String,
}

impl Config {
    pub fn new(path: &str) -> Option<Self> {
        toml::from_str(&fs::read_to_string(path).ok()?).ok()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        toml::from_str(s).ok()
    }

    pub fn get_name(&self) -> &str {
        return &self.name;
    }
}

impl Level {
    pub fn play(&self, config: &Config, num: usize) -> () {
        reset_screen(config, num);
        slow_print(
            &format!("{}", &self.name.bold().magenta()),
            Duration::from_secs_f32(0.1),
        );
        print!("\n\n");

        slow_print(
            &format!("{}\n", &self.prompt.clone().cyan()),
            Duration::from_secs_f32(0.05),
        );

        let key = &self.key.trim().to_lowercase();

        loop {
            slow_print("input> ", Duration::from_secs_f32(0.1));
            let og_input: String = read!("{}\n");
            let input = og_input.trim().to_lowercase();
            if &input == key {
                slow_print(
                    &format!("\"{}\" is correct!\n", og_input.bright_green().bold()),
                    Duration::from_secs_f32(0.1),
                );
                sleep(Duration::from_secs_f32(0.5));
                break;
            } else {
                slow_print(
                    &format!("\"{}\" is incorrect\n", og_input.red()),
                    Duration::from_secs_f32(0.1),
                );
            }
        }
    }
}

fn reset_screen(config: &Config, num: usize) -> () {

    execute!(
        stdout(), 
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All), 
    ).unwrap();

    println!("{}", config.get_name().bright_blue().bold());
    for level in config.levels.iter().take(num) {
        println!("{}", level.name.green());
    }
}


fn slow_print(s: &str, delay: Duration) -> () {
    let mut stdout = stdout();
    for c in s.chars() {
        sleep(delay);
        print!("{}", c);
        stdout.flush().unwrap();
    }
}
