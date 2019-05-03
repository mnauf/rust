fn main() {
    let s1=String::from("I am a string");
    let (length,string) = calc_length(s1);
    println!("The length of given string {}, is {}",string,length)

}
fn calc_length(x:String)->(usize,String){
    let length = x.len();
    let string = x;
    (length,string)
}
