pub fn fibonacci_dynamic(n: i32) -> (i32, i32) {
    if n == 0 {
        return (0, 1);
    }

    let (a, b) = fibonacci_dynamic(n - 1);
    (b, a + b)
}

// challenge: fibonacci dynamic tail_recursive

fn fibonacci_dynamic_tail_recursive(n: i32, a: i32, b: i32) -> (i32, i32) {
    if n == 0 {
        return (a, b);
    }

    fibonacci_dynamic_tail_recursive(n - 1, b, a + b)
}

fn main() {
    let n = 10;
    let (a, _b) = fibonacci_dynamic(n);
    println!("The {}th Fibonacci number is {}", n, a);

    let (a, _b) = fibonacci_dynamic_tail_recursive(n, 0, 1);
    println!("The {}th Fibonacci number is {}", n, a);
}
