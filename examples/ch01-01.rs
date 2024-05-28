pub enum Rest<T, E> {
    Thing(T),
    Error(E),
}

fn divide(a: i32, b: i32) -> Rest<i32, String> {
    if b == 0 {
        Rest::Error("Division by zero".to_string())
    } else {
        Rest::Thing(a / b)
    }
}

fn main() {
    let a = divide(10, 0);

    if let Rest::Thing(v) = a {
        println!("Result: {}", v);
    } else {
        println!("Error");
    }
}
