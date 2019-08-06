fn greetings() {
    println!("hello world");
}
fn closures() {
    let x = greetings;
    x();
}

fn main() {
    closures();
}