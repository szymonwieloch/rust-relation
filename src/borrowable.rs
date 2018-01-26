use Relation;
use std::borrow::{Borrow, BorrowMut};

pub struct Borrowable<T, U> where T: Borrow<_> {
    val: T
}
/*
impl<'a, T> Relation<'a, T> for Borrowable<T> {
    fn get(&'a self) -> &'a T {
        unimplemented!()
    }
}
*/