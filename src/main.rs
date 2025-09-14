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

    // Memory management
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1); // This would cause a compile-time error
    println!("{}", s2);

    let s3 = String::from("world");
    let s4 = s3.clone(); // s3 is cloned to s4
    println!("s3 = {}, s4 = {}", s3, s4);

    let num1 = 10;
    let num2 = num1; // num1 is copied to num2
    println!("num1 = {}, num2 = {}", num1, num2);

    // Ownership with functions
    fn take_ownership(s: String) {
        println!("Taking ownership of: {}", s);
    }
    let s = String::from("ownership");
    take_ownership(s);
    // println!("{}", s); // This would cause a compile-time error
    let num3 = 20;
    fn make_copy(n: i32) {
        println!("Making a copy of: {}", n);
    }
    make_copy(num3);
    println!("num3 = {}", num3); // num3 is still valid

    // Stack vs Heap
    let stack_var = 42; // stored on the stack
    let heap_var = String::from("Hello, heap!"); // stored on the heap
    println!("Stack variable: {}", stack_var);
    println!("Heap variable: {}", heap_var);

    // Slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4]; // slice of the array
    println!("Array slice: {:?}", slice);

    let string = String::from("Hello, world!");
    let string_slice = &string[0..5]; // slice of the string
    println!("String slice: {}", string_slice);

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);   
    println!("The first element of the tuple is: {}", tup.0);
    println!("The second element of the tuple is: {}", tup.1);
    println!("The third element of the tuple is: {}", tup.2);

    // Arrays
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    let arr3 = [3; 5];
    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    println!("arr3: {:?}", arr3);
    println!("The first element of arr1 is: {}", arr1[0]);
    println!("The length of arr1 is: {}", arr1.len());

    // Loops
    for i in 1..6 {
        println!("For loop iteration: {}", i);
    }
    let mut count = 0;
    while count < 5 {
        println!("While loop count: {}", count);
        count += 1;
    }

    let mut n = 0;
    loop {
        n += 1;
        println!("Loop iteration: {}", n);
        if n >= 5 {
            break;
        }
    }

    // Pattern Matching
    let number = 3;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    let boolean = true;
    let binary = match boolean {
        true => 1,
        false => 0,
    };
    println!("The value of binary is: {}", binary);

    // Error Handling
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Vectors
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Vector: {:?}", vec);
    println!("First element: {}", vec[0]);
    println!("Vector length: {}", vec.len());
    for i in &vec {
        println!("Vector element: {}", i);
    }
    vec.pop();
    println!("Vector after pop: {:?}", vec);
}