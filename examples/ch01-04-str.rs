// 'a is optional, but it is a good practice to use it
fn string_find_f<'a>(s: &'a str) -> &'a str {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }

    s
}

// &'static str is a string slice that is guaranteed to be valid for the entire duration of the program
fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "many",
    }
}

fn main() {
    let mut s = " hello ".to_string();
    let p = s.trim();
    let p = p.to_string();

    s.push_str(" world");
    println!("{}", p);

    let fstr = " help me find home";
    let ffstr = string_find_f(fstr);
    println!("{}", ffstr);

    println!("Choose a number: {}", choose_str(3));

    println!("finished!")
}
