use Relation;

pub struct Ref<'a, T> where T: 'a + ?Sized{
    reference: & 'a T
}

impl<'a, T> Relation<'a, T> for Ref<'a, T> where T: ?Sized {
    fn get(&'a self) -> &'a T {
        self.reference
    }
}

impl<'a, T> Ref<'a, T> where T: ?Sized {
    pub fn new(reference: &'a T) ->Self {
        Ref{
            reference
        }
    }
}