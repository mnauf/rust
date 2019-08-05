fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn main() {
    let hey = vec!['a','b','c'];
    println!("{}",largest_char(&hey));
}