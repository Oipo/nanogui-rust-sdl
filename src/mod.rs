#![feature(box_patterns)]

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod widget;
//pub mod screen;
pub mod common;
//pub mod label;
