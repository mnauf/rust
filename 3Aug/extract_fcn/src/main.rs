
    fn largest(list: &[i32])->i32{
        println!("{:?}",list);
        let mut largest = list[0];
        for &i in list {
            if i>largest{
                largest = i;
            };
    };
    largest
}

fn main() {
    let hey = vec![1,3,2,6,90,67,788,12,34,54,32];
    println!("The largest number is: {}",largest(&hey));
}