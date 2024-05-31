fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    n * factorial(n - 1)
}

// The factorial_mem function is generally more performant and faster due
//to its tail-recursive nature, which allows for potential optimizations
//that reduce stack usage and overhead. Even though Rust does not guarantee
// TCO, writing tail-recursive functions is a good practice as
//it can lead to more optimized and safer code when compilers support these optimizations.

fn factorial_mem(n: i32, r: i32) -> i32 {
    if n <= 1 {
        return r;
    }

    factorial_mem(n - 1, n * r)
}

fn main() {}
