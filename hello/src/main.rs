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
// struct Point {
//     x: u32,
//     y: u32
// }
// impl Point {
//     fn x(&self) ->u32{
//         self.x
//     }
// }
// fn main() {
//     let point_1 = Point {
//         x:32,
//         y:56
//     };
//     println!("{}",point_1.x())

// }
#[derive(Debug)]
struct Family<T> {
    gender: String,
    mother:String,
    father:String,
    brothers:u8,
    sisters:u8,
    salary:T
}
impl<T> Family<T> {
    fn mother(&self) ->&String {
        &self.mother
    }
    fn summary(&self)->String{
        if &self.gender == &"male".to_string(){
            let x = "His".to_string();
            let x_1 = "He".to_string();
            format!("{} mother name is {} and father name is {}. {} has altogether {} siblings, where {} is/are brother(s) and {} is/are sisters.",&x,&self.mother,&self.father,&x_1,self.brothers + self.sisters,self.brothers,self.sisters)
        }
        else {
            format!("{} mother name is {} and father name is {}. {} has altogether {} siblings, where {} is/are brother(s) and {} is/are sisters.","Her".to_string(),&self.mother,&self.father,"She".to_string(),self.brothers + self.sisters,self.brothers,self.sisters)
        }
        
    }
        fn salary(&self)->&T{
        &self.salary
    }
}
fn main() {
    let family_1 = Family{
        gender: "female".to_string(),
        mother: "ABC".to_string(),
        father: String::from("meown"),
        brothers:1,
        sisters:3,
        salary: 50000
    };
    println!("{}",family_1.mother());
    println!("{}",family_1.summary());
    println!("The salary is: {}",family_1.salary())
}
