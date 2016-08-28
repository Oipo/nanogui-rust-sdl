#![feature(rc_counts)]

extern crate nanoguirustsdl;
extern crate nanovg;

use nanoguirustsdl::widget::{Widget, WidgetObjRef};
use nanoguirustsdl::theme::Theme;
use std::rc::Rc;
use std::cell::RefCell;



#[test]
fn push_child_test() {
    let mut widget_one = WidgetObjRef::new("one".to_string(), None);
    let widget_two = WidgetObjRef::new("two".to_string(), None);
    let mut widget_three = WidgetObjRef::new("three".to_string(), None);

    {
        widget_one.push_child(widget_two.0.clone());
        assert_eq!(widget_one.id(), "one".to_string());
        assert_eq!(widget_one.children().len(), 1usize);

        match widget_two.parent() {
            Some(val) => { unsafe { assert_eq!((*val).id(), widget_one.id()); } },
            None => panic!("Parent should have value")
        };
    }
    {
        let widget_one_children = widget_one.children();
        assert_eq!(widget_one_children[0].borrow().id(), "two".to_string());
        let ref first_child_borrowed = widget_one_children[0].borrow();
        let widget_parent = first_child_borrowed.parent();

        match widget_parent {
            Some(val) => { unsafe { assert_eq!((*val).id(), widget_one.id()); } },
            None => panic!("Parent should have value")
        };
    }

    widget_three.push_child(widget_two.0.clone());
    assert_eq!(widget_one.children().len(), 0usize);
    {
        let widget_three_children = widget_three.children();
        assert_eq!(widget_three_children.len(), 1usize);
        assert_eq!(widget_three_children[0].borrow().id(), "two".to_string());

        match widget_two.parent() {
            Some(val) => { unsafe { assert_eq!((*val).id(), widget_three.id()); } },
            None => panic!("Parent should have value")
        };
    }
}

#[test]
fn remove_child_test() {
    let mut widget_one = WidgetObjRef::new("one".to_string(), None);
    let widget_two = WidgetObjRef::new("two".to_string(), None);

    {
        widget_one.push_child(widget_two.0.clone());
        assert_eq!(widget_one.id(), "one".to_string());
        assert_eq!(widget_one.children().len(), 1usize);
    }

    widget_one.remove_child_by_id(widget_two.id());
    assert_eq!(widget_one.children().len(), 0usize);
    match widget_two.parent() {
        Some(_) => panic!("Parent should be empty"),
        _ => {}
    }

    match widget_one.remove_child_by_id(widget_two.id()) {
        Some(_) => panic!("No child should be present"),
        _ => {}
    }
}

#[test]
fn parent_test() {
    let widget_one = WidgetObjRef::new("one".to_string(), None);
    let widget_two = WidgetObjRef::new("two".to_string(), Some(widget_one.0.clone()));

    match widget_two.parent() {
        Some(val) => { unsafe { assert_eq!((*val).id(), widget_one.id()); } },
        None => panic!("Parent should have value")
    };
}

#[test]
fn dead_parent_test() {
    println!("dead_parent_test");
    let mut widget_two: Option<WidgetObjRef> = None;
    {
        let widget_one = WidgetObjRef::new("one".to_string(), None);
        println!("strong count outside: {}", Rc::strong_count(&widget_one.0));
        widget_two = Some(WidgetObjRef::new("two".to_string(), Some(widget_one.0.clone())));
        println!("strong count outside: {}", Rc::strong_count(&widget_one.0));
    }

    match widget_two.unwrap().parent() {
        Some(val) => panic!("Parent should not have value"),
        None => {}
    };
}

#[test]
fn theme_test() {
    let theme = Rc::new(RefCell::new(Theme::new_debug()));
    theme.borrow_mut().set_standard_font_size(20);
    let mut widget_one = WidgetObjRef::new("one".to_string(), None);
    widget_one.set_theme(Some(theme.clone()));

    match widget_one.theme() {
        Some(ref val) => {
            assert_eq!(val.borrow().standard_font_size(), theme.borrow().standard_font_size());
        },
        None => panic!("Theme should have value")
    };
}
