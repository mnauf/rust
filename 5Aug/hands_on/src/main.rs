#[derive(Debug)]
// // Single type parameter in struct
// struct Point<T>{
//     x: T,
//     y: T
// }
// fn main() {
//     let x = Point{
//         x: 'c'.to_string(),
//         y: 'z'.to_string()
//     };
// }

// // Mutiple type parameters in struct
// struct Point<T,U>{
//     x: T,
//     y: U
// }
// fn main() {
//     let x = Point{
//         x: 'c'.to_string(),
//         y: 'z'
//     };
// }

// // Single type parameter in methods
// struct Point<T>{
//     x: T,
//     y: T
// }
// impl<T> Point<T>{
//     fn x(&self)-> &T{
//         &self.x
//     }
// } 
// fn main() {
//     let x = Point{
//         x: 'c'.to_string(),
//         y: 'z'.to_string()
//     };
//     println!("{:?}",x.x());
// }

// Multiple type parameter in methods
struct Point<T,U>{
    x: T,
    y: U
}
impl<T,U> Point<T,U>{
    fn x<X,Y>(&self,other: Point<X,Y>)-> Point<&T,Y> {
        Point {
            x: &self.x,
            y: other.y
        }
    }
} 
fn main() {
    let x1 = Point{
        x: 1,
        y: 2
    };
    let x2 = Point{
        x: 3,
        y: 4
    };
    println!("{:?}",x1.x(x2));
    println!("{:?}",x1);
}