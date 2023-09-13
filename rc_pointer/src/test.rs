enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let _f = Rc::new(Cons(10, Rc::new(Cons(6, Rc::new(Nil)))));
    let _e = Rc::new(Cons(10, Rc::new(Nil)));
    let _d = Rc::new(Cons(10, Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))))));
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}