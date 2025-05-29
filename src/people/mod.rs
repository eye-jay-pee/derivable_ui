use std::fmt;

pub mod widgets;

pub struct Person {
    first: String,
    last: String,
    age: i32,
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}, {}", self.first, self.last, self.age)
    }
}
impl Person {
    pub fn new(first: &str, last: &str, age: i32) -> Self {
        Person {
            first: String::from(first),
            last: String::from(last),
            age: age,
        }
    }
}
