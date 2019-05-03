// fn main() {
//     let mut design = String::from("");
//     let mut x = 0;
//     while x<=5{
//         loop{
//             let mut y = 0;
//             design.push_str("*");
//             y=y+1;
//             println!("{}",design);
//             if y==x{
//                 break
//             }
//         }    
//         x=x+1;
//     }
        
//     }
fn main(){
    let mut a = String::from("*");
    let mut x = 0;
    loop {
        println!("{}",a);
        x=x+1;
        a.push_str("*");
        if x==6{
            break
        }
    }
}