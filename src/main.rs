use std::{env, io};

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("Welcome to the 3n+1 calculating program!");
    let starting_number: u128 = get_input("What number would you like to start at?").trim().parse().unwrap();

    loop{
        
    }
}

fn is_odd(num: u128) -> bool{
    if num % 2 == 0 {return false}
    else{return true}
}


fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}
