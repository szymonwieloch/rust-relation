pub trait Relation<'a, T> where T: ?Sized{
    #[inline]
    fn get(&'a self) -> &'a T;
}
