mod commands;

use std::io::{stdin, stdout, Write};
use std::fs;
use std::env;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use fltk::{app, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    wind.end();
    wind.show();
    

    app.run().unwrap();

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

            Some("sys") => {
                let mut sys = System::new_all();
                sys.refresh_all();

                println!(
                    "{0: <15} | {1: <15} | {2: <15} | {3: <15}",
                    "System", "blanks", "comments", "code",
                );

                println!("{0} {1: <14}|", "", sys.host_name().unwrap());
                println!("{0} {1: <14}|", "", sys.name().unwrap());
                println!("{0} {1: <14}|", "", sys.kernel_version().unwrap());
                println!("{0} {1} {2: <9}|", "", "UPT:", sys.uptime());
            }

            _ => println!("command not known")
        }

    }
}
