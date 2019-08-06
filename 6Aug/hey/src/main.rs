fn main() {
    let string_1 = "abfc";
    let string_2 = "xyz";
    let result = longest(string_1,string_2);
    println!("The longest of the two string is: {}",result);
}
fn longest<'a>(x: &'a str,y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}