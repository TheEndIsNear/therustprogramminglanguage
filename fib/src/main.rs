use std::io;

fn main() {
    println!("Input a value for n:");
    let n: i32 = read_input();
    let result = fib(n);
    println!("{}", result);
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read input");
    input.trim().parse().expect("Expected to receive an integer")
}

fn fib(n: i32) -> i32 {
    if n == 1 || n == 0 {
        return n;
    } else {
        return fib(n - 2) + fib(n -1);
    }
}
