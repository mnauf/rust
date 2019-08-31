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
// let score = scores.get(team_name); // This method only accepts the reference
// println!("{:?}",score);
// }

// Program 5
// fn main() {
// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

// let team_name = String::from("Blue");
// let score = scores.get(&team_name);
// println!("{:?}",score);
// }

//Program 6
// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// }

//Program 7
// fn main() {
//     use std::collections::HashMap;

// let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);

//     for (key, value) in &scores {
//         println!("{}: {}", key, value);
//     }
// }

// Program 8

// use std::collections::HashMap;
// fn main() {
// let mut scores = HashMap::new();
// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Blue"), 25);

// println!("{:?}", scores);
// }

// Program 9
// fn main() {
// use std::collections::HashMap;
// let mut scores = HashMap::new();
// scores.insert(String::from("Blue"), 10);
// scores.entry(String::from("Yellow")).or_insert(50);
// scores.entry(String::from("Blue")).or_insert(50);
// println!("{:?}", scores);
// }
// Program 10
// use std::collections::HashMap;
// fn main() {
//     let text = "hello world wonderful world";

//     let mut map = HashMap::new();

//     for word in text.split_whitespace() {
//         let count = map.entry(word).or_insert(0);
//         *count += 1;
//     }

//     println!("{:?}", map);
// }
