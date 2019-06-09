extern crate todo;
extern crate diesel;

use self::todo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();
    println!("\nOk! Let's write a task (Press {} when finished)\n", EOF);
        let mut task = String::new();
    stdin().read_to_string(&mut task).unwrap();
    let tasktask = create_task(&connection, &task);
    println!("\nSaved draft {} with id {}", &task, tasktask.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";