// TODO: Remove once core development is done
#![allow(unused_variables, unused_imports)]

use std::io;

use crate::prelude::*;

mod utils;
mod prelude;

fn main() -> Result<(), ()> {
    println!("🦀 KVS... A simple key value store written in Rust");

    let runstate: bool = true;

    // REPL 
    while runstate {
        let mut user_input: String = String::new();
        let _ = io::stdin().read_line(&mut user_input).is_ok();
        proc_input(&user_input);
    }
    
    Ok(())
}

fn proc_input(input: &String) {
    match input.as_str().trim() {
        ".exit" => std::process::exit(0),
        _ => println!("{input}")
    }
}