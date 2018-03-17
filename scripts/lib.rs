#![cfg_attr(feature = "unstable", feature(core_intrinsics))]
#![cfg_attr(feature = "unstable", feature(on_unimplemented))]
#![feature(ascii_ctype)]
#![feature(conservative_impl_trait)]
#![feature(const_fn)]
#![feature(mpsc_select)]
#![feature(plugin)]
#![feature(proc_macro)]
#![feature(splice)]
#![feature(string_retain)]
#![deny(unsafe_code)]
#![allow(non_snake_case)]
#![doc = "The script crate contains all matters DOM."]
#![plugin(script_plugins)]
#![cfg_attr(not(feature = "unrooted_must_root_lint"), allow(unknown_lints))]
#![allow(unused)]

extern crate angle;
extern crate app_units;
extern crate audio_video_metadata;
extern crate base64;
#[macro_use]
extern crate bitflags;
extern crate bluetooth_traits;
extern crate byteorder;
extern crate canvas_traits;
extern crate caseless;
extern crate chrono;
extern crate cookie as cookie_rs;
#[macro_use]
extern crate cssparser;
#[macro_use]
extern crate deny_public_fields;
extern crate devtools_traits;
#[macro_use]
extern crate dom_struct;
#[macro_use]
extern crate domobject_derive;
extern crate encoding_rs;
extern crate euclid;
extern crate fnv;
extern crate gleam;
extern crate half;
#[macro_use]
extern crate html5ever;
#[macro_use]
extern crate hyper;
extern crate hyper_serde;
extern crate image;
extern crate ipc_channel;
#[macro_use]
extern crate jstraceable_derive;
#[macro_use]
extern crate lazy_static;
extern crate libc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate malloc_size_of;
#[macro_use]
extern crate malloc_size_of_derive;
extern crate metrics;
#[macro_use]
extern crate mime;
extern crate mime_guess;
extern crate mitochondria;
#[macro_use]
extern crate mozjs as js;
extern crate msg;
extern crate net_traits;
extern crate num_traits;
extern crate offscreen_gl_context;
extern crate parking_lot;
extern crate phf;
#[macro_use]
extern crate profile_traits;
extern crate ref_filter_map;
extern crate ref_slice;
extern crate regex;
extern crate script_layout_interface;
extern crate script_traits;
extern crate selectors;
extern crate serde;
extern crate servo_allocator;
extern crate servo_arc;
#[macro_use]
extern crate servo_atoms;
extern crate servo_config;
extern crate servo_geometry;
extern crate servo_rand;
extern crate servo_url;
extern crate smallvec;
#[macro_use]
extern crate style;
extern crate style_traits;
extern crate swapper;
extern crate time;
#[cfg(any(target_os = "macos", target_os = "linux", target_os = "windows"))]
extern crate tinyfiledialogs;
extern crate unicode_segmentation;
extern crate url;
extern crate utf8;
extern crate uuid;
extern crate webrender_api;
extern crate webvr_traits;
extern crate xml5ever;

#[macro_use]
pub mod task;
pub mod dom;
pub mod script_thread;

pub mod mem;
