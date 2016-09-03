extern crate nanoguirustsdl;
extern crate nanovg;

use nanoguirustsdl::widget::{Widget, WidgetObj};
use nanoguirustsdl::widget_container::{push_child, remove_child_by_child};
use nanoguirustsdl::theme::Theme;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;


#[test]
fn push_and_remove_child_test() {
    let mut widget_one = Rc::new(RefCell::new(WidgetObj::new("one".to_string())));
    let mut widget_two = Rc::new(RefCell::new(WidgetObj::new("two".to_string())));
    let mut widget_three = Rc::new(RefCell::new(WidgetObj::new("three".to_string())));

    {
        push_child(widget_one.clone(), widget_two.clone());
        assert_eq!(widget_one.borrow().id(), "one".to_string());
        assert_eq!(widget_one.borrow().children().len(), 1usize);

        if let Some(ref parent) = widget_two.borrow().parent() {
            if let Some(ref val) = parent.upgrade() {
                assert_eq!(val.borrow().id(), widget_one.borrow().id())
            } else {
                panic!("parent should be upgradable");
            }
        } else {
            panic!("parent should have value");
        }
    }
    {
        let children = widget_one.borrow().children();
        let first_child = children[0].borrow();
        assert_eq!(first_child.id(), "two".to_string());

        if let Some(ref parent) = first_child.parent() {
            if let Some(ref val) = parent.upgrade() {
                assert_eq!(val.borrow().id(), widget_one.borrow().id())
            } else {
                panic!("parent should be upgradable");
            }
        } else {
            panic!("parent should have value");
        }
    }

    remove_child_by_child(widget_one.clone(), widget_two.clone());
    push_child(widget_three.clone(), widget_two.clone());
    assert_eq!(widget_one.borrow().children().len(), 0usize);
    {
        let children_one = widget_one.borrow().children();
        let children_three = widget_three.borrow().children();
        let first_child = children_three[0].borrow();
        assert_eq!(children_one.len(), 0usize);
        assert_eq!(children_three.len(), 1usize);
        assert_eq!(first_child.id(), "two".to_string());

        let widget_two_borrowed = widget_two.borrow();
        if let Some(ref parent) = widget_two_borrowed.parent() {
            if let Some(ref val) = parent.upgrade() {
                assert_eq!(val.borrow().id(), widget_three.borrow().id())
            } else {
                panic!("parent should be upgradable");
            }
        } else {
            panic!("parent should have value");
        }
    }
}

#[test]
fn parent_test() {
    let mut widget_one = Rc::new(RefCell::new(WidgetObj::new("one".to_string())));
    let mut widget_two = Rc::new(RefCell::new(WidgetObj::new("two".to_string())));
    push_child(widget_one.clone(), widget_two.clone());

    let widget_two_borrowed = widget_two.borrow();
    if let Some(ref parent) = widget_two_borrowed.parent() {
        if let Some(ref val) = parent.upgrade() {
            assert_eq!(val.borrow().id(), widget_one.borrow().id())
        } else {
            panic!("parent should be upgradable");
        }
    } else {
        panic!("parent should have value");
    }
}

#[test]
fn dropped_parent_test() {
    println!("smh");
    let mut widget_two: Option<Rc<RefCell<WidgetObj>>>;
    {
        let mut widget_one = Rc::new(RefCell::new(WidgetObj::new("one".to_string())));
        let mut temp_widget_two = Rc::new(RefCell::new(WidgetObj::new("two".to_string())));
        //println!("address one: {:p}", &widget_one);
        //println!("address two: {:p}", &temp_widget_two);
        push_child(widget_one.clone(), temp_widget_two.clone());
        widget_two = Some(temp_widget_two);
    }

    let unwrapped = widget_two.unwrap();
    let unwrapped_borrowed = unwrapped.borrow();
    //println!("address unwrap: {:p}", &unwrapped);

    if let Some(ref parent) = unwrapped_borrowed.parent() {
        panic!("parent should be None");
    }
}

#[test]
fn theme_test() {
    let theme = Rc::new(RefCell::new(Theme::new_debug()));
    theme.borrow_mut().set_standard_font_size(20);
    let mut widget_one = WidgetObj::new("one".to_string());
    widget_one.set_theme(Some(theme.clone()));

    match widget_one.theme {
        Some(ref val) => {
            assert_eq!(val.borrow().standard_font_size(), theme.borrow().standard_font_size());
        },
        None => panic!("Theme should have value")
    };
}
