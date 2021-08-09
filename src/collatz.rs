//the acutal algorythm
pub fn run(mut number: u128) -> (u32, u128){
    //max number
    let mut max_number: u128 = 0;
    // total times that the loop has run before arriving to 1
    let mut steps: u32 = 0;

    while number != 1{
        //why no ++ :(
        steps += 1;
        //find the max
        if number > max_number {
            max_number = number;
        }
        // if the number is odd, multiply by 3 and add 1
        if is_odd(number){
            number = 3 * number + 1;
        }else{
            //if the number is even, divide it by 2
            number = number / 2;
        }

    }
    
    (steps, max_number)
}

//get is odd number just makes the code above more clean idk
fn is_odd(num: u128) -> bool{
    if num % 2 == 0 {return false}
    else{return true}
}