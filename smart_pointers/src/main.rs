use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct SmartPointer {
    data: String,
}

impl Drop for SmartPointer {
    fn drop(&mut self) {
        println!("Dropping SmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

use crate::List::{Cons, Nil};

fn main() {
    dropping();
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(4, Box::new(Nil))))));
    let _ka = MyBox::new(String::from("ka"));
    println!("list {:?} ", list);
}

fn dropping() {
    let _x: SmartPointer = SmartPointer {
        data: String::from("my stuff"),
    };
    let _y: SmartPointer = SmartPointer {
        data: String::from("other stuff"),
    };
    println!("SmartPointer created: the data. {} {}", _x.data, _y.data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refs() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
