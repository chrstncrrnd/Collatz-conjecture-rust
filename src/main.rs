use std::io;
use std::time::Instant;

fn main() {

    println!("Welcome to the 3n+1 calculating program!");
    //get the input for max
    let mut number: u128 =  get_input("What number would you like to start at?").trim().parse().unwrap();
    //max number
    let mut max: u128 = 0;
    // total times that the loop has run before arriving to 1
    let mut iterations: u128 = 0;
    let start_time = Instant::now();

    while number != 1{
        //why no ++ :(
        iterations += 1;
        //find the max
        if number > max {
            max = number;
        }
        // if the number is odd, multiply by 3 and add 1
        if is_odd(number){
            number = 3 * number + 1;
        }else{
            //if the number is even, divide it by 2
            number = number / 2;
        }

    }

    let duration = start_time.elapsed();
    //print the results
    println!("The program went over a total of {} total numbers before returning to 1", iterations);
    println!("The maximum that the number got to was {}", max);
    println!("Time elapsed: {} milliseconds", duration.as_millis())

}

//get is odd number just makes the code above more clean idk
fn is_odd(num: u128) -> bool{
    if num % 2 == 0 {return false}
    else{return true}
}

//get input function because it just makes stuff easier
fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_nothing) => {println!("error with input");},
    }
    input.trim().to_string()
}
