use Relation;
use std::borrow::Borrow;


pub struct Boxed<T> where T: ?Sized {
    b: Box<T>
}

impl<'a, T> Relation<'a, T> for Boxed<T> where T: ?Sized{
    fn get(&'a self) -> &'a T {
        self.b.borrow()
    }
}