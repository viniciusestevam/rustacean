fn main() {
    let number = 10;
    println!("Fibonacci sequence of {}", number);

    for n in 0..number {
        println!("{}", fib(n))
    }
}

fn fib(number: u32) -> u32 {
    match number {
        0 | 1 => 1,
        _ => fib(number - 1) + fib(number - 2),
    }
}
