fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let mut new =(1,2,3);
    println!("{},{},{}",new.0,new.1,new.2);
    new = (4,5,6);
    println!("{},{},{}",new.0,new.1,new.2);

}