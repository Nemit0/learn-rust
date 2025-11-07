// Reference solution (not compiled by default).

#[derive(Debug, PartialEq, Eq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn cons(head: i32, tail: List) -> List {
        List::Cons(head, Box::new(tail))
    }

    pub fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, tail) => 1 + tail.len(),
        }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut out = Vec::new();
        fn go(lst: &List, out: &mut Vec<i32>) {
            match lst {
                List::Nil => {}
                List::Cons(h, t) => {
                    out.push(*h);
                    go(t, out);
                }
            }
        }
        go(self, &mut out);
        out
    }
}

use std::rc::Rc;

pub fn make_rc(value: i32) -> Rc<i32> { Rc::new(value) }
pub fn clone_rc(rc: &Rc<i32>) -> Rc<i32> { Rc::clone(rc) }
pub fn strong_count(rc: &Rc<i32>) -> usize { Rc::strong_count(rc) }

