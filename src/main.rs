mod commands;

use std::io::{stdin, stdout, Write};
use std::fs;


fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let trimmed_input = input.trim();
        let command = trimmed_input.split_whitespace().next();
        let params = trimmed_input.split_whitespace().skip(1);
        let params_vector: Vec<&str> = params.collect();


        match command {
            Some("cat") => {
                if !params_vector.is_empty() {
                    commands::cat(&params_vector[0]);
                }
            },

            Some("ls") => {
                if !params_vector.is_empty() {
                    commands::ls(&params_vector[0]);
                } else {
                    commands::ls(".");
                }
            },

            Some("touch") => {
                if !params_vector.is_empty() {
                    commands::touch(&params_vector[0]);
                }
            },

            Some("cd") => {
                if !params_vector.is_empty() {
                    commands::cd(&params_vector[0]);
                }
            }

            _ => println!("command not known")
        }

    }
}
