// fn closures(){
//     let mut one = 1;
//     let plus_one = |x: u32| -> u32 {x+one};
//     println!("{} + 1: {}",3,plus_one(3));
//     let borrow_one = &mut one;
// }
// fn main() {
//     closures();
// }
fn closures() {
    let add_three = |mut x:i32| -> i32 {x+3};
    let f = 12;
    println!("f={:?}",add_three(f));
}
fn main() {
    closures();
}
