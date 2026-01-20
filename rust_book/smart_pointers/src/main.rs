use std::ops::Deref;
use crate::List::{Cons, Nil};
use std::rc::Rc;

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}");
}

struct CustromSmartPointer {
    data: String,
}

impl Drop for CustromSmartPointer {
    fn drop(&mut self) {
        println!("Drop CstmSmrtPoint w {}", self.data);
    }
}

enum ListRc {
    Cons2(i32, Rc<ListRc>),
    Nil2,
}
use crate::ListRc::{Cons2, Nil2};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    let c = CustromSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustromSmartPointer {
        data: String::from("other stuff"),
    };
    drop(d);
    println!("Start of two CstSmrtPnt");


    let u = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("Count after u = {}", Rc::strong_count(&u));
    let v = Cons2(3, Rc::clone(&u));
    println!("Count after v = {}", Rc::strong_count(&u));
    {
        let w = Cons2(4, Rc::clone(&u));
        println!("Count after w = {}", Rc::strong_count(&u));
    }
    println!("Count after w out of scope = {}", Rc::strong_count(&u));
}
