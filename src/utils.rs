use std::io;
//get input function because it just makes stuff easier
pub fn get_input(prompt: &str) -> String{
    println!("{}",prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_nothing) => {println!("error with input");},
    }
    input.trim().to_string()
}