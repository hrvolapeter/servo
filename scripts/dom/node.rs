use dom::bindings::conversions::IDLInterface;
use dom::bindings::utils::DOMClass;
use dom_struct::dom_struct;
use dom::bindings::reflector::Reflector;

#[dom_struct]
pub struct Node {
    a: Reflector,
}

impl IDLInterface for Node {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!();
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        unimplemented!();
    }
}