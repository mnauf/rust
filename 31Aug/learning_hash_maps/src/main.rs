// Program 1
// fn main() {
// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);
// println!("{:?}",scores);
// }
// Program 2
// fn main() {
// use std::collections::HashMap;

// let teams  = vec![String::from("Blue"), String::from("Yellow")];
// let initial_scores = vec![10, 50];

// let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
// println!("{:?}",scores);
// }
// Program 3

// fn main() {
// use std::collections::HashMap;

// let field_name = String::from("Favorite color");
// let field_value = String::from("Blue");

// let mut map = HashMap::new();
// map.insert(field_name, field_value);
// // field_name and field_value are invalid at this point, try using them and
// // see what compiler error you get!
// println!("{}",field_name);
// }

// Program 4
// fn main() {
// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

// let team_name = String::from("Blue");
// let score = scores.get(&team_name);
// println!("{:?}",score);
// }

#![allow(unused_variables)]
fn main() {
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
}