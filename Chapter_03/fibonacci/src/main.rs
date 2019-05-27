use std::io;

fn main() {
    println!("Fibonacci calculator.");

    println!("Please enter which number within the Fibonacci sequence you want to calculate.");

    let mut input_fib = String::new();

    io::stdin().read_line(&mut input_fib)
        .expect("Failed to read line from stdin");

    let input_fib : u32 = input_fib.trim().parse()
        .expect("No valid input!");

    println!("The number in the Fibonacci sequence on index {} is {}", input_fib, fib_calc(input_fib));
}

fn fib_calc(index: u32) -> u32 {
    if index == 0 {
        0
    } else if index == 1 {
        1
    } else if index == 2 {
        1 
    } else {
        fib_calc(index-1) + fib_calc(index-2)
    }
}
