#![feature(plugin)]
#![plugin(interpolate_idents)]

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[macro_use]
pub mod common;
pub mod resources;
pub mod theme;
pub mod widget;
pub mod widget_container;
pub mod screen;
pub mod label;
