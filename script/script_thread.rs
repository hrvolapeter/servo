/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! The script thread is the thread that owns the DOM in memory, runs JavaScript, and spawns parsing
//! and layout threads. It's in charge of processing events for all same-origin pages in a frame
//! tree, and manages the entire lifetime of pages in the frame tree from initial request to
//! teardown.
//!
//! Page loads follow a two-step process. When a request for a new page load is received, the
//! network request is initiated and the relevant data pertaining to the new page is stashed.
//! While the non-blocking request is ongoing, the script thread is free to process further events,
//! noting when they pertain to ongoing loads (such as resizes/viewport adjustments). When the
//! initial response is received for an ongoing load, the second phase starts - the frame tree
//! entry is created, along with the Window and Document objects, and the appropriate parser
//! takes over the response body. Once parsing is complete, the document lifecycle for loading
//! a page runs its course and the script thread returns to processing events in the main event
//! loop.

use dom::element::Element;
use std::rc::Rc;
use dom::customelementregistry::CallbackReaction;
use dom::customelementregistry::CustomElementDefinition;
use typeholder::TypeHolderTrait;
use std::marker::PhantomData;

#[derive(JSTraceable)]
// ScriptThread instances are rooted on creation, so this is okay
#[allow(unrooted_must_root)]
pub struct ScriptThread<TH: TypeHolderTrait> {
	_p: PhantomData<TH>
}

pub enum MainThreadScriptMsg { }

impl<TH: TypeHolderTrait> ScriptThread<TH> {
    pub fn enqueue_callback_reaction(element: &Element<TH>,
                                     reaction: CallbackReaction,
                                     definition: Option<Rc<CustomElementDefinition>>) {
        unimplemented!();
    }
}