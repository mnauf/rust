// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &str) -> String {
//     format!("the two strings are {} and {}",x.as_string(),y.as_string())
// }

// // fn main() {
// //     let string1 = String::from("I love Pakistan");
// //     let result;
// //     {
// //     let string2 = String::from("Hello world");
// //     let result = longest(string1.as_str(),string2.as_str());
    
// //     }
// //     println!("The longest is: {}",result);
// // }
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{:?}",i);
}
