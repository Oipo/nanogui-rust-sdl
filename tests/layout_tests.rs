extern crate nanoguirustsdl;
extern crate nanovg;

use nanoguirustsdl::widget::{Widget, WidgetObj};
use nanoguirustsdl::widget_container::push_child;
use nanoguirustsdl::layout::{BoxLayout, Orientation};
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn preferred_size_no_layout_test() {
    let widget_one = Rc::new(RefCell::new(WidgetObj::new("one".to_string())));
    let vg: nanovg::Context = nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);

    widget_one.borrow_mut().set_size((10, 10));

    let result = widget_one.borrow().preferred_size(&vg);
    assert_eq!(result, (10u32, 10u32));
}

#[test]
fn preferred_size_without_children_test() {
    let widget_one = Rc::new(RefCell::new(WidgetObj::new("one".to_string())));
    let mut layout = BoxLayout::new(Orientation::Horizontal);
    let vg: nanovg::Context = nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);

    layout.margin = 1;
    layout.spacing = 1;
    widget_one.borrow_mut().set_layout(Some(Box::new(layout)));
    widget_one.borrow_mut().set_size((10, 10));

    let result = widget_one.borrow().preferred_size(&vg);
    assert_eq!(result, (2u32, 2u32));
}

#[test]
fn preferred_size_with_children_test() {
    let widget_one = Rc::new(RefCell::new(WidgetObj::new("one".to_string())));
    let widget_two = Rc::new(RefCell::new(WidgetObj::new("two".to_string())));
    let mut layout = BoxLayout::new(Orientation::Horizontal);
    let vg: nanovg::Context = nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);

    layout.margin = 1;
    layout.spacing = 1;
    widget_one.borrow_mut().set_layout(Some(Box::new(layout)));
    push_child(widget_one.clone(), widget_two.clone());
    widget_one.borrow_mut().set_size((10, 10));
    widget_two.borrow_mut().set_size((10, 10));
    widget_two.borrow_mut().set_visible(false);

    let mut result = widget_one.borrow().preferred_size(&vg);
    assert_eq!(result, (2u32, 2u32));

    widget_two.borrow_mut().set_visible(true);

    result = widget_one.borrow().preferred_size(&vg);
    assert_eq!(result, (12u32, 12u32));

    widget_two.borrow_mut().set_fixed_size((5, 5));

    result = widget_one.borrow().preferred_size(&vg);
    assert_eq!(result, (7u32, 7u32));
}

#[test]
fn preferred_size_with_children_of_children_test() {
    let widget_one = Rc::new(RefCell::new(WidgetObj::new("one".to_string())));
    let widget_two = Rc::new(RefCell::new(WidgetObj::new("two".to_string())));
    let widget_three = Rc::new(RefCell::new(WidgetObj::new("three".to_string())));
    let mut layout_one = BoxLayout::new(Orientation::Horizontal);
    let mut layout_two = BoxLayout::new(Orientation::Horizontal);
    let vg: nanovg::Context = nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);

    layout_one.margin = 1;
    layout_one.spacing = 1;
    layout_two.margin = 1;
    layout_two.spacing = 1;
    widget_one.borrow_mut().set_layout(Some(Box::new(layout_one)));
    widget_two.borrow_mut().set_layout(Some(Box::new(layout_two)));
    push_child(widget_one.clone(), widget_two.clone());
    push_child(widget_two.clone(), widget_three.clone());
    widget_one.borrow_mut().set_size((10, 10));
    widget_two.borrow_mut().set_size((10, 10));
    widget_three.borrow_mut().set_size((10, 10));

    let result = widget_one.borrow().preferred_size(&vg);
    assert_eq!(result, (14u32, 14u32));
}
