#![allow(unused_variables)]
fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
            email: String::from("m.naufil1@gmail.com"),
            username: String::from("Muhammad Naufil"),
            active: true,
            sign_in_count: 1,
        };
    let user2 = User {
        ..user1
            // email: String::from("m.naufil1@gmail.com"),
            // username: String::from("Muhammad Naufil"),
            // active: true,
            // sign_in_count: 1,
        };
    // let user1 = build_user(String::from("m.naufil1@gmail.com"),String::from("Muhammad Naufil"));
    println!("{}",user1.active);
    println!("{}",user1.username);
}
