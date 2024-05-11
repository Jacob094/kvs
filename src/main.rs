// TODO: Remove once core development is done
#![allow(unused_variables, unused_imports)]

use std::{env::args, io};

use crate::prelude::*;

mod prelude;
mod storage;
mod utils;

fn main() {

    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        println!(r#"
        KVS - Terminal Based Key Value Store
        Usage:
            
            -r REPL Mode
            -h 
            == Topics ==
            list             -- List Topics
            new <topic_name> -- create new topics
        "#);
    }


    // println!("🦀 KVS... A simple key value store written in Rust");

    // REPL
    if args.contains(&"-r".to_string()) {
        println!("#KVS REPL");
        loop {
            let mut user_input: String = String::new();
            let _ = io::stdin().read_line(&mut user_input).is_ok();
            proc_input(&user_input);
    }
        
    }
}

fn proc_input(input: &String) {
    match input.as_str().trim() {
        ".exit" => std::process::exit(0),
        _ => println!("{input}"),
    }
}
