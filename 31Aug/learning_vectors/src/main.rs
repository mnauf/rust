// Program 1
// fn main() {
//     let v = vec![1,2,3];
//     println!("{}",v);
// }
// Program 2
// fn main() {
//     let v = vec![1,2,3,'a']; // expected integer found char
// }
// Program 3
// fn main() {
//     let v = vec!['a',1,2,3]; //expected char found integer
// }
// fn main() {
//     let v:Vec<i32>;
//     v = vec![1,2,3];
//     println!("{:?}",v);
// }
fn main() {
    let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
}