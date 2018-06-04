/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::node::Node;
use dom::element::AttributeMutation;
use dom::attr::Attr;
use typeholder::TypeHolderTrait;

/// Trait to allow DOM nodes to opt-in to overriding (or adding to) common
/// behaviours. Replicates the effect of C++ virtual methods.
pub trait VirtualMethods<TH: TypeHolderTrait> {
    fn attribute_mutated(&self, attr: &Attr<TH>, mutation: AttributeMutation) {
        unimplemented!();
    }
}

/// Obtain a VirtualMethods instance for a given Node-derived object. Any
/// method call on the trait object will invoke the corresponding method on the
/// concrete type, propagating up the parent hierarchy unless otherwise
/// interrupted.
pub fn vtable_for<TH: TypeHolderTrait>(node: &Node<TH>) -> &VirtualMethods<TH> {
    unimplemented!();
}