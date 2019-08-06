// use std::thread;
// use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(||{
//         for i in 0..9 {
//             println!("From spawn thread counting: {}",i);
//             thread::sleep(Duration::from_millis(1));
//         };
//     });
//     for i in 0..5 {
//         println!("From main thread counting: {}",i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     handle.join().unwrap();
// }
fn trying(x: u32)->u32{
    x
}
fn main()
{
    let x = 32;
    let y = trying(x);
    println!("{}",y);
}