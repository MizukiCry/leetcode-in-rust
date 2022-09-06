use std::{cell::{Cell, RefCell}, rc::Rc, borrow::BorrowMut};

fn main() {

    {
        let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));

        let s1 = s.clone();
        let s2 = s.clone();
        // let mut s2 = s.borrow_mut();
        s2.borrow_mut().push_str(", on yeah!");

        println!("{:?}\n{:?}\n{:?}", s, s1, s2);
    }

    /*let mut x = Rc::new(Cell::new(1));
    let t = x.clone();
    t.borrow_mut() += 1;

    let mut func = || {
        
        *t.borrow_mut().get_mut() += 1;
    };

    println!("{}", x.get());
    func();*/

}