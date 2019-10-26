use std::io;
fn main() {
    println!(">");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Enter String correctly");
    let mut result = String::from("");
    for c in input.chars(){
        match c.to_string().as_ref() {
            // '*'=>result = multiply(&input),
            "+"=> result = add(&input),
            // '-'=>result = subtract(&input),
            // '/'=>result = divide(&input),
            _ => print!(""),
        }
    };
    println!("{}",result);
    }

fn add(text: &String)->String{
    let mut string:Vec<char> = text.trim().chars().collect();
    
}