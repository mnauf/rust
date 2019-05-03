fn main() {
    //convert f into c
    let f = 180.0;
    let c = ((f - 32.0) * 5.0 ) / 9.0;
    println!("The temperature in celsius is: {}", c);
    //Generate nth fibonacci sequence
    let mut tup = (1,2,3);
    tup.1 = 3;
    println!("The tup includes: {:?}",tup);
    fn fibonacci(n:usize) {
        let mut sum=[0,1,2,3,4,5,6,7,8,9,10];
        for index in 2..n
            {
            sum[index] = sum[index-1] + sum[index-2];
        }
        println!("{:?}",sum);
    }
    fibonacci(11)
}