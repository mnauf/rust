#[derive(Debug)]
struct family{
    mother: String,
    father: String,
    eldest_brother: String,
    youngest_sister: String
}
trait Summary{
    fn summarize(&self)->String{
        format!("Read more")
    }
}
impl Summary for family{
    fn summarize(&self)->String{
        format!("{} and {} bear {} and {}",self.father,self.mother,self.eldest_brother,self.youngest_sister)
    }
}

fn main() {
    let family_1 = family {
        mother: String::from("Nudrat"),
        father: String::from("Raheel"),
        eldest_brother: String::from("Sarosh"),
        youngest_sister: String::from("Anum")
    };
    println!("{}",family_1.summarize());
    // fn acceptance(t: &impl Summary)->String{
    //     format!("Accepted")      
    // }
    fn acceptance<T>(item: &T)->String
    where T: Summary {
        format!("{}",item.summarize())
    }
    println!("{}",acceptance(&family_1));
    println!("{:#?}",family_1);
    let s = 3.to_string();
    println!("{}",s);
}