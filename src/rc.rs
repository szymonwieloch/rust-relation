use Relation;
use std::rc::Rc as StdRc;
use std::borrow::Borrow;

pub struct Rc<T> {
    rc: StdRc<T>
}

impl<'a, T> Relation<'a, T> for Rc<T> {
    fn get(&'a self) -> &'a T {
        self.rc.borrow()
    }
}