/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use dom_struct::dom_struct;
use dom::bindings::inheritance::Castable;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::utils::DOMClass;
use js::jsapi::JSContext;
use dom::bindings::root::DomRoot;
use js::jsapi::JSObject;
use dom::bindings::reflector::DomObject;
use dom::bindings::error::ErrorInfo;
use js::jsapi::HandleValue;
use dom::bindings::reflector::Reflector;
use typeholder::TypeHolderTrait;
use std::marker::PhantomData;


#[dom_struct]
pub struct GlobalScope<TH: TypeHolderTrait> {
    a: Reflector,
    _p: PhantomData<TH>
}

impl<TH: TypeHolderTrait> GlobalScope<TH> {
     pub fn get_cx(&self) -> *mut JSContext {
        unimplemented!();
    }

    #[allow(unsafe_code)]
    pub unsafe fn from_context(cx: *mut JSContext) -> DomRoot<Self> {
        unimplemented!();
    }

    #[allow(unsafe_code)]
    pub unsafe fn from_object(obj: *mut JSObject) -> DomRoot<Self> {
        unimplemented!();
    }

    pub fn incumbent() -> Option<DomRoot<Self>> {
        unimplemented!();
    }

    #[allow(unsafe_code)]
    pub fn from_reflector<T: DomObject>(reflector: &T) -> DomRoot<Self> {
        unimplemented!();
    }

    pub fn perform_a_microtask_checkpoint(&self) {
        unimplemented!();
    }

    pub fn report_an_error(&self, error_info: ErrorInfo, value: HandleValue) {
        unimplemented!();
    }
}

impl<TH: TypeHolderTrait> IDLInterface for GlobalScope<TH> {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!();
    }
}

impl<TH: TypeHolderTrait> PartialEq for GlobalScope<TH> {
    fn eq(&self, other: &GlobalScope<TH>) -> bool {
        unimplemented!();
    }
}