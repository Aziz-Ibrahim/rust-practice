use std::io;

fn main() {
    let x = 8;
    let y = 2;
    let z = x + y;
    println!("The sum of {} and {} is {}", x, y, z);

    const SECONDS_IN_A_MINUTE: u32 = 60;
    const MINUTES_IN_AN_HOUR: u32 = 60;
    const HOURS_IN_A_DAY: u32 = 24;
    const SECONDS_IN_A_DAY: u32 = SECONDS_IN_A_MINUTE * MINUTES_IN_AN_HOUR * HOURS_IN_A_DAY;
    println!("There are {} seconds in a day.", SECONDS_IN_A_DAY);

    let mut input = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Please enter a valid number");
    let squared = num * num;
    println!("The square of {} is {}", num, squared);
}
