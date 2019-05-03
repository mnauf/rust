fn main() {
    // let hello = "hello";
    // let hello1= hello;
    // println!("{}",hello);

    // let hello = String::from("hello");
    // let hello1 = hello.clone();
    // println!("{}",hello);

//     let string = "Hello, World";
//     println!("{:p}",string);
//     foo(string);
// }

// fn foo(string:&str) {
//     println!("{:p}",string);
prime(12);
prime_test(6);
}


fn prime(n:i32){
    for i in 2..(n/2) {
        if n%i==0 {
            println!("Not a prime number");
            break;
        }
    }
}
fn prime_test(n:i32)->bool{
    let result = for i in 2..(n/2) {
        if n%i==0 {
            break false;
        };
    };
}