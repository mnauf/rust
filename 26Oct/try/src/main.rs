// mod greetings;
// mod teachers_name; 
// fn main() {
//     greetings::morning_greetings();
//     teachers_name::chemisty_teacher();
//     teachers_name::maths_teacher();
//     teachers_name::physics_teacher();
//     greetings::noon_greetings();
//     greetings::evening_greetings();
//     greetings::night_greetings();

// }


// mod mixed;
// fn main() {
//     mixed::greetings::evening_greetings();
//     mixed::teachers_name::maths_teacher()
// }


// mod mixed;
// use mixed::greetings;
// use mixed::teachers_name;
// fn main() {
//     greetings::evening_greetings();
//     teachers_name::maths_teacher()
// }


// mod mixed;
// use mixed::greetings::evening_greetings;
// use mixed::teachers_name::maths_teacher;
// fn main() {
//     evening_greetings();
//     maths_teacher()
// }


mod mixed;
use mixed::{greetings::evening_greetings,teachers_name::maths_teacher};
fn main() {
    evening_greetings();
    maths_teacher()
}