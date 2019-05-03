fn main() {
    const X:u32=123;
    println!("Hello world {}",X);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces are welldone: {}",spaces);
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}",guess);
}