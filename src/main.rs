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

    // Type casting with user input
    println!("Enter an integer to convert to float:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let int_input: i64 = input2.trim().parse().expect(
        "Please enter a valid number"
    );
    let float_input: f64 = int_input as f64;
    println!(
        "You entered integer: {}, which is float: {}",
        int_input, float_input
    );

    // Boolean operations
    let bool1 = 2<3;
    let bool2 = 5==5;
    let bool3 = 7>10;
    let bool4 = (2 as f64) < 3.5;
    println!("2 < 3: {}", bool1);
    println!("5 == 5: {}", bool2);
    println!("7 > 10: {}", bool3);
    println!("2.0 < 3.5: {}", bool4);

    // Control Flow
    let mut number = String::new();
    println!("Enter a number to check if it's even or odd:");
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number: i32 = number.trim().parse().expect(
        "Please enter a valid number"
    );
    if number % 2 == 0 {
        println!("{} is even.", number);
    } else {
        println!("{} is odd.", number);
    }

    // More complex condition
    println!("Enter your age:");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("Failed to read line");
    let age: u32 = age_input.trim().parse().expect(
        "Please enter a valid number"
    );
    if age < 13 {
        println!("You are a child.");
    } else if age < 20 {
        println!("You are a teenager.");
    } else if age < 65 {
        println!("You are an adult.");
    } else {
        println!("You are a senior.");
    }


    // Functions
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
    greet("Alice");
    greet("Bob");

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let sum = add(5, 7);
    println!("The sum of 5 and 7 is {}", sum);

    // Nested functions
    fn outer_function(x: i32) -> i32 {
        fn inner_function(y: i32) -> i32 {
            y + 2
        }
        inner_function(x) * 2
    }
    let result = outer_function(3);
    println!("The result of the nested function is {}", result);


    // Expression vs Statement
    let expr = {let x = 3; x + 1}; // expression
    println!("The value of the expression is {}", expr);

    let stmt = {
        let y = 5; println!("This is a statement, y is {}", y);
    }; // statement
    stmt; // executing the statement block


}