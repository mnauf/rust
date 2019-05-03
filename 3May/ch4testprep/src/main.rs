// fn main() {
//     let s1 = gives_ownership();
//     let s2 = takes_returns_ownership(s1);
//     println!("{}",s2)
// }
// fn gives_ownership() -> String {
//     let s1 =String::from("Hello world");
//     s1
// }
// fn takes_returns_ownership(x1: String) -> String {
//     x1
// }

// fn main() {
//     let s1 = "Hello world";
//     let s2 = s1;
//     println!("{:p},{:p}",s1,s1);
// }

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
    println!("{}",s)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}