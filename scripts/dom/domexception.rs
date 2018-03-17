/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::reflector::{reflect_dom_object, Reflector};
use dom::bindings::root::DomRoot;
use dom::bindings::str::DOMString;
use dom::globalscope::GlobalScope;
use dom_struct::dom_struct;
use dom::bindings::codegen::Bindings::DOMExceptionBinding::DOMExceptionMethods;
use dom::bindings::utils::DOMClass;
use dom::bindings::conversions::IDLInterface;


#[repr(u16)]
#[derive(Clone, Copy, Debug, JSTraceable, MallocSizeOf)]
pub enum DOMErrorName {
    IndexSizeError = 1,
    HierarchyRequestError = 2,
    WrongDocumentError = 3,
    InvalidCharacterError = 4,
    NoModificationAllowedError = 5,
    NotFoundError = 6,
    NotSupportedError = 7,
    InUseAttributeError = 8,
    InvalidStateError = 9,
    SyntaxError = 10,
    InvalidModificationError = 11,
    NamespaceError = 12,
    InvalidAccessError = 13,
    SecurityError = 14,
    NetworkError = 15,
    AbortError = 16,
    TypeMismatchError = 17,
    QuotaExceededError = 18,
    TimeoutError = 19,
    InvalidNodeTypeError = 20,
    DataCloneError = 21,
}

#[dom_struct]
pub struct DOMException {
    a: Reflector,
}

impl DOMException {
    fn new_inherited(code: DOMErrorName) -> DOMException {
        unimplemented!();
    }

    pub fn new(global: &GlobalScope, code: DOMErrorName) -> DomRoot<DOMException> {
        unimplemented!();
    }
}

impl DOMExceptionMethods for DOMException {
    fn Stringifier(&self) -> DOMString {
        unimplemented!();
    }

    fn Code(&self) -> u16 {
        unimplemented!();
    }

    fn Name(&self) -> DOMString {
        unimplemented!();
    }

    fn Message(&self) -> DOMString {
        unimplemented!();
    }
}

impl IDLInterface for DOMException {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!();
    }
}

impl PartialEq for DOMException {
    fn eq(&self, other: &DOMException) -> bool {
        unimplemented!();
    }
}