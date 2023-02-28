mod commands;

use std::io::{stdin, stdout, Write};
use std::fs;
use std::env;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

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

            Some("sysinfo") => {
                let mut sys = System::new_all();
                sys.refresh_all();
                println!("Disks:");
                for disk in sys.disks() {
                    println!("{:?}", disk);
                }
                println!("=> networks:");
                for (interface_name, data) in sys.networks() {
                    println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
                }
                println!("=> components:");
                for component in sys.components() {
                    println!("{:?}", component);
                }
                println!("=> system:");
                println!("total memory: {} bytes", sys.total_memory());
                println!("used memory : {} bytes", sys.used_memory());
                println!("total swap  : {} bytes", sys.total_swap());
                println!("used swap   : {} bytes", sys.used_swap());
                println!("System name:             {:?}", sys.name());
                println!("System kernel version:   {:?}", sys.kernel_version());
                println!("System OS version:       {:?}", sys.os_version());
                println!("System host name:        {:?}", sys.host_name());
                println!("NB CPUs: {}", sys.cpus().len());
            }

            _ => println!("command not known")
        }

    }
}
