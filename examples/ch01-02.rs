#[derive(Debug, Copy, Clone)]
pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }

        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut s = Stepper {
        curr: 0,
        step: 2,
        max: 10,
    };

    for i in s {
        println!("{}", i);
    }

    while let Some(i) = s.next() {
        println!("{}", i);
    }

    loop {
        match s.next() {
            Some(i) => println!("{}", i),
            None => break,
        }
    }
}
