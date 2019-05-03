fn main() {
    let s1=String::from("I am naufil");
    let text_string = length(&s1);
    println!("The lenth of string {}, is: {}",s1,text_string);
}

fn length(x:&String)-> (usize){
    (x.len())
}
