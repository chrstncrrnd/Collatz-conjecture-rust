use std::time::Instant;
mod utils;
mod finder;
mod collatz;

fn main() {
    println!("Welcome to the 3n+1 calculating program!");
    loop{
        let user_option = utils::get_input("What version would you like to run?\n'n': normal\n'f': finder\n");

        if user_option == "n"{
            //get the input for max
            let starting_number: u128 = utils::get_input("What number would you like to start at?").trim().parse().unwrap();
            let start_time = Instant::now();
            let (steps, max) = collatz::run(starting_number);
            let duration = start_time.elapsed();
            //print the results
            println!("The program went over a total of {} steps before returning to 1", steps);
            println!("The maximum that the number got to was {}", max);
            println!("Time elapsed: {} milliseconds", duration.as_millis())

        }else if user_option == "f"{
            finder::finder();
        }

        let run_again = utils::get_input("\nWould you like to run again?\n'y': yes\n'n': no");
        if run_again == "n"{break;}
    }

}