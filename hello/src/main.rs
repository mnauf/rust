#[derive(Debug)]
struct Point<T,U>{
    x: T,
    y:U
}
fn main() {
    let point_1 = Point{
        x:5.0,
        y:3
    };
    println!("{:?}",point_1)

}