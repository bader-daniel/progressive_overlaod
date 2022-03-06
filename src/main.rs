//use crate::workout_data::*;
use Progressive_Overload::*;


fn main() {
    let mut workout1 = Workout::new(21, 2, 31);


    //println!("date of workout: {:?}", workout1.date.unwrap());

    //workout1.completed_excercises = Some(SingleExcercise::new());
    workout1.generate_datasets();

    println!("{:?}", workout1);

}
