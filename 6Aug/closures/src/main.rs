fn closures(){
    let plus_one = |x: u32| -> u32 {x+1};
    println!("{} + 1: {}",3,plus_one(3));
}
fn main() {
    closures();
}