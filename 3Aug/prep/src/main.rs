fn main() {
    let h = vec![4,3,2,1];
    println!("{:?}",h);
    let mut largest = h[0];
    for number in h
    {
        if number > largest{
            largest = number;
        };
    };
    println!("The largest number in list is: {}",largest);
}