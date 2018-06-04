use dom::bindings::conversions::IDLInterface;
use dom::bindings::utils::DOMClass;
use dom_struct::dom_struct;
use dom::bindings::reflector::Reflector;
use typeholder::TypeHolderTrait;
use std::marker::PhantomData;

#[dom_struct]
pub struct Node<TH: TypeHolderTrait> {
    a: Reflector,
    _p: PhantomData<TH>,
}

impl<TH: TypeHolderTrait> IDLInterface for Node<TH> {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!();
    }
}

impl<TH> PartialEq for Node<TH> {
    fn eq(&self, other: &Node<TH>) -> bool {
        unimplemented!();
    }
}