// fn main() {
    // struct User {
    //     name: String,
    //     email: String,
    //     active: bool,
    //     password: u64,
    // };
    // let user1 = User {
    //     name: String::from("Muhammad Naufil"),
    //     email: String::from("m.naufil1@gmail.com"),
    //     active: true,
    //     password: 12345,
    // };
    // println!("{}",user1.name);
    //     fn build_user(email: String, username: String,active: bool, sign_in_count: u64) -> User {
    //     User {
    //         email: String,
    //         username: String,
    //         active: bool,
    //         sign_in_count: u64,
    //     }
    // };
    // let user1 = build_user(String::from("m.naufil1@gmail.com"),String::from("Muhammad Naufil"),true,32);

    // #![allow(unused_variables)]
    // fn main() {

//     let mut user1 = build_user(
//         String::from("m.naufil1@gmail.com"),
//         String::from("Muhammad Naufil"),
//     );
//     user1.active = false;
//     println!("{},{}", user1.email,user1.active);
// }


// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

#![allow(unused_variables)]
fn main() {
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let mut black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
black.0=1;
println!("{}",black.0);
}