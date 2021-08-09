use std::time::Instant;
use crate::collatz;
use crate::utils;

pub fn finder(){
    //get the input for max
    let number: u128 = utils::get_input("Up to what number would you like to run the for loop for? ").trim().parse().unwrap();
    let start_time = Instant::now();

    let mut max_steps: u32 = 0;
    let mut max: u128 = 0;

    let mut current_step_ammount: u32;


    for i in 1..number{
        current_step_ammount = collatz::run(i).0;
        if current_step_ammount > max_steps{
            max_steps = current_step_ammount;
            max = i;
        }
    }

    print!("Largest ammount of steps was: {} on the number {} \nAnd it took a total of: {} milliseconds.", max_steps, max, start_time.elapsed().as_millis())

}