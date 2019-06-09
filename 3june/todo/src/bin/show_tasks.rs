extern crate todo;
extern crate diesel;

use self::todo::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use todo::schema::tasks::dsl::*;

    let connection = establish_connection();
    let results = tasks.limit(5)
        .load::<Task>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.task);
        println!("----------\n");
    }
}