/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Native representation of JS Promise values.
//!
//! This implementation differs from the traditional Rust DOM object, because the reflector
//! is provided by SpiderMonkey and has no knowledge of an associated native representation
//! (ie. dom::Promise). This means that native instances use native reference counting (Rc)
//! to ensure that no memory is leaked, which means that there can be multiple instances of
//! native Promise values that refer to the same JS value yet are distinct native objects
//! (ie. address equality for the native objects is meaningless).

use dom_struct::dom_struct;
use devtools_traits::WorkerId;
use dom::bindings::error::Error;
use js::conversions::ToJSValConvertible;
use dom::bindings::reflector::Reflector;
use typeholder::TypeHolderTrait;
use std::marker::PhantomData;
use std::cell::Cell;

#[dom_struct]
pub struct Promise<TH: TypeHolderTrait> {
    reflector: Reflector,
    _p: PhantomData<TH>
}

impl<TH: TypeHolderTrait> Promise<TH> {
    pub fn resolve_native<T>(&self, val: &T) where T: ToJSValConvertible {
        unimplemented!();
    }

    pub fn reject_error(&self, error: Error<TH>) {
        unimplemented!();
    }
}