extern crate relation;
use relation::{Relation, Ref, Box as RelBox};

struct Object {
    pub val: i32
}

struct Dependent<R> where for <'a> R: Relation<'a, Object> {
    object: R
}

impl<R> Dependent<R> where for <'a> R: Relation<'a, Object>{
    pub fn new(object: R) -> Self{
        Self{object}
    }
}


fn main(){
    //create object
    let obj = Object{val: 5};
    let relation = Ref::<Object>::new(&obj);
    let dep = Dependent::new(relation);
}