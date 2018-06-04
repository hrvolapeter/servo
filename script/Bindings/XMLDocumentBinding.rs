

pub use self::XMLDocumentBinding::{XMLDocumentMethods, Wrap};
pub mod XMLDocumentBinding {
    use dom::bindings::root::DomRoot;
    use dom::xmldocument::XMLDocument;
    use js::jsapi::JSObject;
    use std::ptr::NonNull;
    use dom::bindings::str::DOMString;
    use js::jsapi::JSContext;
    use dom::globalscope::GlobalScope;
    use typeholder::TypeHolderTrait;

    pub unsafe fn Wrap<TH: TypeHolderTrait>(cx: *mut JSContext, scope: &GlobalScope, object: Box<XMLDocument<TH>>) -> DomRoot<XMLDocument<TH>> {


        let raw = Box::into_raw(object);
        DomRoot::from_ref(&*raw)
    }

    pub trait XMLDocumentMethods {
        fn SupportedPropertyNames(&self) -> Vec<DOMString>;
        unsafe fn NamedGetter(&self, cx: *mut JSContext, name: DOMString) -> Option<NonNull<JSObject>>;
    }
} // mod XMLDocumentBinding


