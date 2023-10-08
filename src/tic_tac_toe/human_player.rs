use std::io::{self, Write};
extern crate regex;
use anyhow::anyhow;
use regex::Regex;

pub struct HumanPlayer {
    mark: char,
}

impl HumanPlayer {
    pub fn new(mark: char) -> Self {
        HumanPlayer { mark }
    }

    pub fn get_mark(&self) -> char {
        self.mark
    }

    pub fn get_position(&self) -> anyhow::Result<(usize, usize)> {
        println!(
            "Player {}, enter two numbers representing a position in the format 'row col'",
            self.mark
        );

        let mut user_input = String::from("");

        io::stdout().flush()?;

        io::stdin().read_line(&mut user_input)?;

        let position = user_input.trim();

        let valid_pattern = r"^\d\s\d$";

        let re = Regex::new(valid_pattern)?;

        if re.is_match(position) {
            let position = self
                .parse_position(position)
                .ok_or(anyhow!("Failed to parse position"));

            return position;
        } else {
            Err(anyhow!(
                "Invalid position. The position must be in the format 'row col'."
            ))
        }
    }

    fn parse_position(&self, position: &str) -> Option<(usize, usize)> {
        let parts: Vec<_> = position.trim().split(" ").collect();

        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                return Some((num1, num2));
            }
        }

        None
    }
}
