// #[derive(Debug)]
// struct Point<T,U>{
//     x: T,
//     y:U
// }
// fn main() {
//     let point_1 = Point{
//         x:5.0,
//         y:3
//     };
//     println!("{:?}",point_1)

// }
struct Point {
    x: u32,
    y: u32
}
impl Point {
    fn x(&self) ->u32{
        self.x
    }
}
fn main() {
    let point_1 = Point {
        x:32,
        y:56
    };
    println!("{}",point_1.x())

}