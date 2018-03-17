/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use html5ever::{LocalName, Namespace};
use script_thread::ScriptThread;
use std::rc::Rc;
use dom::bindings::reflector::Reflector;
use dom::node::Node;
use dom::bindings::str::DOMString;

pub enum Mutation {
    Attribute { name: LocalName, namespace: Namespace, old_value: DOMString },
}

#[dom_struct]
pub struct MutationObserver {
    a: Reflector,
}

impl MutationObserver {
    pub fn queue_a_mutation_record(target: &Node, attr_type: Mutation) {
        unimplemented!();
    }
}
