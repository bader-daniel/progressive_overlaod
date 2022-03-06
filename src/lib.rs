// One workout in it's entirety
#[derive(Debug)]
pub struct Workout {
    pub date: Option<Date>,
    pub completed_excercises: Option<SingleExcercise>,
}

impl Workout {
    pub fn new(y: u16, m: u8, d: u8) -> Workout {
        // Regular constructor, create a new Struct object
        Self {
            date: Option::from(Date(y, m, d)),
            completed_excercises: None,
        }
    }

    pub fn generate_datasets(&mut self) {
        self.completed_excercises = Some(SingleExcercise::new());
    }


}

// A single type of excercise as part of a workout
#[derive(Debug)]
pub struct SingleExcercise {
    pub excercise_name: Option<ExercicesByName>,
    pub completed_sets: Vec<Option<SetsStruct>>,
}

impl SingleExcercise {
    fn new() -> SingleExcercise {
        Self {
            excercise_name: None,
            completed_sets: vec![None],
        }
    }
}



// The date of the workout
#[derive(Debug)]
pub struct Date(u16, u8, u8);

// Sets of a particular excercise: id, repetitions, weight
#[derive(Debug)]
pub struct SetsStruct(u8, u8, usize);

// All the various excercises to choose from
#[derive(Debug)]
pub enum ExercicesByName {
    Squats(String),
    Deadlifts(String),
    BenchPress(String),
    PullDowns(String),
    OverheadPress(String),
}


#[cfg(test)]
mod tests {
}


