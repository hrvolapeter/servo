/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use dom_struct::dom_struct;
use dom::bindings::reflector::Reflector;
use dom::bindings::utils::WindowProxyHandler;
use typeholder::TypeHolderTrait;

#[dom_struct]
// NOTE: the browsing context for a window is managed in two places:
// here, in script, but also in the constellation. The constellation
// manages the session history, which in script is accessed through
// History objects, messaging the constellation.
pub struct WindowProxy<TH: TypeHolderTrait> {
    a: Reflector<TH>,
}


pub fn new_window_proxy_handler() ->WindowProxyHandler {
        unimplemented!();
    }