/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use dom::bindings::reflector::Reflector;
use typeholder::TypeHolderTrait;
use std::marker::PhantomData;


/// The number of times we are allowed to see spurious `requestAnimationFrame()` calls before
/// falling back to fake ones.
///
/// A spurious `requestAnimationFrame()` call is defined as one that does not change the DOM.
const SPURIOUS_ANIMATION_FRAME_THRESHOLD: u8 = 5;

/// The amount of time between fake `requestAnimationFrame()`s.
const FAKE_REQUEST_ANIMATION_FRAME_DELAY: u64 = 16;

/// <https://dom.spec.whatwg.org/#document>
#[dom_struct]
pub struct Document<TH: TypeHolderTrait + 'static> {
    a: Reflector,
    phantom: PhantomData<TH>,
}

#[derive(Clone, Copy, Eq, JSTraceable, MallocSizeOf, PartialEq)]
pub enum HasBrowsingContext {
    No,
    Yes,
}

#[derive(Clone, Copy, Debug, JSTraceable, MallocSizeOf, PartialEq)]
pub enum IsHTMLDocument {
    HTMLDocument,
    NonHTMLDocument,
}

/// Specifies the type of focus event that is sent to a pipeline
#[derive(Clone, Copy, PartialEq)]
pub enum FocusType {
    Element,
    // The first focus message - focus the element itself
    Parent,  // Focusing a parent element (an iframe)
}

#[derive(MallocSizeOf, PartialEq)]
pub enum DocumentSource {
    FromParser,
    NotFromParser,
}

pub enum TouchEventResult {
    Processed(bool),
    Forwarded,
}

pub enum FireMouseEventType {
    Move,
    Over,
    Out,
}
