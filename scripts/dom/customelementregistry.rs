use html5ever::{LocalName, Namespace};
use dom::bindings::str::DOMString;
use std::mem;

pub enum CallbackReaction {
    Connected,
    Disconnected,
    AttributeChanged(LocalName, Option<DOMString>, Option<DOMString>, Namespace),
}

#[derive(Clone, JSTraceable, MallocSizeOf)]
pub struct CustomElementDefinition {
}