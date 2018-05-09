/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Element nodes.
use dom_struct::dom_struct;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::utils::DOMClass;
use dom::bindings::reflector::Reflector;
use std::fmt;
use style;
use dom::attr::Attr;
use dom::bindings::root::DomRoot;
use html5ever::{Namespace, LocalName};
use std::rc::Rc;
use style::attr::AttrValue;
use dom::bindings::str::DOMString;
use dom::customelementregistry::CustomElementDefinition;

// TODO: Update focus state when the top-level browsing context gains or loses system focus,
// and when the element enters or leaves a browsing context container.
// https://html.spec.whatwg.org/multipage/#selector-focus

#[dom_struct]
pub struct Element {
    a: Reflector,
}

impl Element {
    pub fn get_attribute(&self, namespace: &Namespace, local_name: &LocalName) -> Option<DomRoot<Attr>> {
        unimplemented!();
    }

    pub fn will_mutate_attr(&self, attr: &Attr) {
        unimplemented!();
    }

    pub fn set_custom_element_definition(&self, definition: Rc<CustomElementDefinition>) {
        unimplemented!();
    }

    pub fn get_custom_element_definition(&self) -> Option<Rc<CustomElementDefinition>> {
        unimplemented!();
    }

    pub fn parse_attribute(&self,
                           namespace: &Namespace,
                           local_name: &LocalName,
                           value: DOMString)
                           -> AttrValue {
        unimplemented!();
    }
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!();
    }
}

#[derive(Clone, Copy)]
pub enum AttributeMutation<'a> {
    /// The attribute is removed.
    /// <https://dom.spec.whatwg.org/#attribute-is-removed>
    Removed,
    Set(Option<&'a AttrValue>),
}

impl IDLInterface for Element {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!();
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        unimplemented!();
    }
}