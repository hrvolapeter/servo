/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::XMLDocumentBinding::{self, XMLDocumentMethods};
use dom::bindings::inheritance::Castable;
use dom::bindings::reflector::reflect_dom_object;
use dom::bindings::root::DomRoot;
use dom::bindings::str::DOMString;
use dom::document::{Document, DocumentSource, HasBrowsingContext, IsHTMLDocument};
use dom::window::Window;
use dom_struct::dom_struct;
use js::jsapi::JSContext;
use js::jsapi::JSObject;
use mime::Mime;
use script_traits::DocumentActivity;
use servo_url::{MutableOrigin, ServoUrl};
use std::ptr::NonNull;
use typeholder::TypeHolderTrait;
use dom::bindings::conversions::IDLInterface;
use dom::bindings::utils::DOMClass;
use dom::bindings::codegen::Bindings::DocumentBinding::DocumentMethods;
use std::marker::PhantomData;
// https://dom.spec.whatwg.org/#xmldocument
#[dom_struct]
pub struct XMLDocument<TH: TypeHolderTrait> {
    document: Document<TH>,
}

impl<TH: TypeHolderTrait> IDLInterface for XMLDocument<TH> {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!();
    }
}

impl<TH: TypeHolderTrait> XMLDocument<TH> {
    fn new_inherited(window: &Window<TH>,
                     has_browsing_context: HasBrowsingContext,
                     url: Option<ServoUrl>,
                     origin: MutableOrigin,
                     is_html_document: IsHTMLDocument,
                     content_type: Option<Mime>,
                     last_modified: Option<String>,
                     activity: DocumentActivity,
                     source: DocumentSource) -> XMLDocument<TH> {
        XMLDocument {
            document: Document::<TH>::new_inherited(),
        }
    }

    pub fn new(window: &Window<TH>,
               has_browsing_context: HasBrowsingContext,
               url: Option<ServoUrl>,
               origin: MutableOrigin,
               doctype: IsHTMLDocument,
               content_type: Option<Mime>,
               last_modified: Option<String>,
               activity: DocumentActivity,
               source: DocumentSource)
               -> DomRoot<XMLDocument<TH>> {
        let doc = reflect_dom_object(
            Box::new(XMLDocument::new_inherited(
                window,
                has_browsing_context,
                url,
                origin,
                doctype,
                content_type,
                last_modified,
                activity,
                source
            )),
            window,
            XMLDocumentBinding::Wrap
        );
        doc
    }
}

impl<TH: TypeHolderTrait> XMLDocumentMethods for XMLDocument<TH>{
    // https://html.spec.whatwg.org/multipage/#dom-tree-accessors:supported-property-names
    fn SupportedPropertyNames(&self) -> Vec<DOMString> {
        self.upcast::<Document<TH>>().SupportedPropertyNames()
    }

    #[allow(unsafe_code)]
    // https://html.spec.whatwg.org/multipage/#dom-tree-accessors:dom-document-nameditem-filter
    unsafe fn NamedGetter(&self, _cx: *mut JSContext, name: DOMString) -> Option<NonNull<JSObject>> {
        self.upcast::<Document<TH>>().NamedGetter(_cx, name)
    }
}
