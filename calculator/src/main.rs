use std::env::{args, Args};


fn main() {
    let mut args: Args = args();

    let first = args.nth(1).unwrap(); // getting first argument and it gets unwrapped 
    let second = args.nth(0).unwrap(); // after unwrapping second argument becomes 1st
    let third = args.nth(0).unwrap(); // just like upper scenario


    let first_number = first.parse::<f32>().unwrap(); // typecasting
    let second_number = third.parse::<f32>().unwrap();
    let operator = second.parse::<char>().unwrap();

    let result:f32 = operate(operator, first_number, second_number);

    println!("Result is {}",result);

}

fn operate(operator: char, first_number:f32, second_number:f32) -> f32{
// using match, panic!() 
    match operator { 
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator used")
    }
}
