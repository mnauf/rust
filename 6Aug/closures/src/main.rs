fn plus_one(x:u32)->u32{
    x+1
}
fn closures(){
    let x= plus_one;
    println!("{}",x(2));
}
fn main() {
    closures();
}