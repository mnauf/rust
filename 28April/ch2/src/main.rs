use std::io;
fn main() {
    let mut inp_value = String::new();
    io::stdin().read_line(&mut inp_value);
    let inp_value: i32 = inp_value.trim().parse().expect("Please type a number");
    let answer = cube(inp_value);
    println!("Input value is: {}",inp_value);
    println!("Cube of input value {} is: {}",inp_value,answer);
}
fn cube(x:i32)-> (i32){
    (x*x*x)
}