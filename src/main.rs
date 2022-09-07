use std::{cell::RefCell, rc::Rc};

fn main() {

    let x = Rc::new(RefCell::new(1));
    let mut t = x.clone().borrow_mut();
    *t += 1;

    let mut func = || {
        *t += 1;
    };

    for _ in 0..2 {
        println!("{}", x.borrow());
        func();
    }

}