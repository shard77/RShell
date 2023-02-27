use std::io::{stdin, stdout, Write};
use std::env;
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
                    let contents = fs::read_to_string(params_vector[0])
                        .expect("Should have been able to read the file");

                    println!("🐈‍⬛ Cat Output:\n{contents}");
                }
            },

            _ => println!("command not known")
        }



    }
}
