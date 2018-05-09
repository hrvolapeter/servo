#[macro_use]
pub mod macros;

pub mod attr;
pub mod customelementregistry;
pub mod element;
pub mod mutationobserver;
pub mod node;
pub mod virtualmethods;
pub mod window;
pub mod bindings;
pub mod document;
pub mod globalscope;
pub mod windowproxy;
pub mod abstractworker;
pub mod domexception;
pub mod promise;
pub mod reflect;

pub mod types {
    use dom_struct::dom_struct;

    #[cfg(not(target_env = "msvc"))]
    include!("../InterfaceTypes.rs");
    #[cfg(target_env = "msvc")]
    include!(concat!(env!("OUT_DIR"), "/build/InterfaceTypes.rs"));
    
}
