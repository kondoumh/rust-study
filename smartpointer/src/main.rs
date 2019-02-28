enum List {
    Cons(i32, Box<List>),
    Nil,
}

// use List::{Cons, Nil};

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomeSmartPointer {
    data: String,
}

impl Drop for CustomeSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    // let list = Cons(1,
    //     Box::new(Cons(2,
    //         Box::new(Cons(3,
    //             Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let m = MyBox::new(x);

    assert_eq!(5, *m);

    let n = MyBox::new(String::from("Rust"));
    hello(&n);

    let c = CustomeSmartPointer{ data: String::from("my stuff") };
    let d = CustomeSmartPointer{ data: String::from("other stuff") };
    println!("CustomSmartPointer created");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}