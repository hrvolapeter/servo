/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use js::jsapi::{JSRuntime, JS_RequestInterruptCallback};
use js::rust::Runtime;

#[derive(Clone, Copy)]
pub struct SharedRt {
    rt: *mut JSRuntime,
}

impl SharedRt {
    pub fn new(rt: &Runtime) -> SharedRt {
        SharedRt { rt: rt.rt() }
    }

    #[allow(unsafe_code)]
    pub fn request_interrupt(&self) {
        unsafe {
            JS_RequestInterruptCallback(self.rt);
        }
    }
}

#[allow(unsafe_code)]
unsafe impl Send for SharedRt {}
