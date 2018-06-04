use dom_struct::dom_struct;
use dom::bindings::reflector::Reflector;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::utils::DOMClass;
use typeholder::TypeHolderTrait;
use std::marker::PhantomData;

#[dom_struct]
pub struct Window<TH: TypeHolderTrait> {
    a: Reflector,
    _p: PhantomData<TH>,
}

impl<TH: TypeHolderTrait> IDLInterface for Window<TH> {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!();
    }
}

impl<TH: TypeHolderTrait> PartialEq for Window<TH> {
    fn eq(&self, other: &Window<TH>) -> bool {
        unimplemented!();
    }
}