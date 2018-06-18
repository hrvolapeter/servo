use dom::bindings::codegen::Bindings;
use dom::bindings::codegen::PrototypeList::Proxies;
use libc;

pub static mut PROXY_HANDLERS: [*const libc::c_void; 26] = [0 as *const libc::c_void; 26];

/// Create the global vtables used by the generated DOM bindings to implement JS proxies.
pub unsafe fn RegisterProxyHandlers() {
    PROXY_HANDLERS[Proxies::Document as usize] = Bindings::DocumentBinding::DocumentBinding::DefineProxyHandler();
}
