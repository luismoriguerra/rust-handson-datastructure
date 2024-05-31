#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: &str, age: u32) -> Person {
        Person {
            name: name.to_string(),
            age: age,
        }
    }

    pub fn greet(&self) -> String {
        format!(
            "Hello, my name is {} and I am {} years old",
            self.name, self.age
        )
    }

    // mutable borrow
    pub fn age_up(&mut self) {
        self.age += 1;
    }

    // consuming self, or taking ownership of self or destroying self
    pub fn dropme(self) {
        println!("Dropping {:?}", self);
    }
}

// immutable borrow
fn get_age(p: &Person) -> &u32 {
    &p.age
}

fn main() {
    let mut p = Person::new("John", 32);
    p.age_up();

    let age = get_age(&p);

    //cannot borrow `p` as mutable because it is also borrowed as immutable
    // p.age_up();
    println!("Age: {}", age);
    println!("Age: {}", age);

    p.age_up();

    println!("{:?}", p);
}
