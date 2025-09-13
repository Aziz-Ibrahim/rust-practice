use std::io;

fn main() {
    // Variables
    let x = 8;
    let y = 2;
    let z = x + y;
    println!("The sum of {} and {} is {}", x, y, z);

    // Constants
    const SECONDS_IN_A_MINUTE: u32 = 60;
    const MINUTES_IN_AN_HOUR: u32 = 60;
    const HOURS_IN_A_DAY: u32 = 24;
    const SECONDS_IN_A_DAY: u32 = SECONDS_IN_A_MINUTE * MINUTES_IN_AN_HOUR * HOURS_IN_A_DAY;
    println!("There are {} seconds in a day.", SECONDS_IN_A_DAY);

    // Input and Output
    let mut input = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Please enter a valid number");
    let squared = num * num;
    println!("The square of {} is {}", num, squared);

    // Arithmetic Operations
    // Integer operations
    let a = 15;
    let b = 4;
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, a / b);
    println!("{} % {} = {}", a, b, a % b);

    // Floating-point operations
    let c: f64 = 5.0;
    let d: f64 = 2.0;
    println!("{} / {} = {}", c, d, c / d);  
    println!("{} % {} = {}", c, d, c % d);

    // Type Casting
    let int_num: i32 = 10;
    let float_num: f64 = int_num as f64;
    println!("Integer: {}, Float: {}", int_num, float_num);
    let float_num2: f64 = 9.7;
    let int_num2: i32 = float_num2 as i32;
    println!("Float: {}, Integer: {}", float_num2, int_num2);
}
