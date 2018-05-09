use dom_struct::dom_struct;
use dom::bindings::reflector::Reflector;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::utils::DOMClass;

#[dom_struct]
pub struct Window {
    a: Reflector
}

impl IDLInterface for Window {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!();
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Window) -> bool {
        unimplemented!();
    }
}