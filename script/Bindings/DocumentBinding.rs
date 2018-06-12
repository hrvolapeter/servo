pub mod DocumentBinding {
	use libc;
	use dom::bindings::conversions::IDLInterface;
	use dom::document::Document;
	use typeholder::TypeHolderTrait;
	use dom::bindings::utils::DOMClass;
	use dom::bindings::str::DOMString;

	pub unsafe fn DefineProxyHandler() -> *const libc::c_void {
		unimplemented!();
	}

	
	impl<TH: TypeHolderTrait> IDLInterface for Document<TH> {
    #[inline]
    fn derives(class: &'static DOMClass) -> bool {
        unimplemented!()
    }
}

impl<TH: TypeHolderTrait> PartialEq for Document<TH> {
    fn eq(&self, other: &Document<TH>) -> bool {
        unimplemented!()
    }
}
	
}
use typeholder::TypeHolderTrait;
use dom::bindings::str::DOMString;
use js::jsapi::JSObject;
use std::ptr::NonNull;
use js::jsapi::JSContext;

pub struct DocumentReadyState;
pub struct ElementCreationOptions;
pub trait DocumentMethods {
	fn SupportedPropertyNames(&self) -> Vec<DOMString>;
	unsafe fn NamedGetter(&self, _cx: *mut JSContext, name: DOMString) -> Option<NonNull<JSObject>>;
}