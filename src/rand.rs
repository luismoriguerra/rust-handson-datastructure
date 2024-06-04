use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    static ref RG: Mutex<RandGen> = Mutex::new(RandGen::new(34056));
}

pub fn rand(max: usize) -> usize {
    RG.lock().unwrap().next_v(max)
}

pub struct RandGen {
    curr: usize,
    mul: usize,
    inc: usize,
    module: usize,
}

impl RandGen {
    pub fn new(curr: usize) -> Self {
        RandGen {
            curr,
            mul: 56394237,
            inc: 34642349,
            module: 23254544563,
        }
    }

    pub fn next_v(&mut self, max: usize) -> usize {
        self.curr = (self.curr * self.mul + self.inc) % self.module;
        self.curr % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut r = RandGen::new(12);
        for _ in 0..10 {
            println!("{}", r.next_v(100));
        }
    }
}
