extern crate nanovg;

use std::rc::Rc;
use std::cell::RefCell;
use widget::Widget;

pub fn push_child(container: Rc<RefCell<Widget>>, new_child: Rc<RefCell<Widget>>) {
    {
        let child_id: String;
        let mut parent: Option<Rc<RefCell<Widget>>> = None;
        {
            let new_child_borrow = new_child.borrow();
            child_id = new_child_borrow.id().clone();

            if let Some(ref val) = new_child_borrow.parent() {
                if let Some(val_upgraded) = val.upgrade() {
                    parent = Some(val_upgraded);
                }
            }
        }

        if container.borrow().id() == child_id {
            panic!("Do not add a widget to itself.");
        }

        if let Some(ref val) = parent {
            remove_child_by_id(val.clone(), child_id);
        }
    }
    unsafe {
        new_child.borrow_mut().set_parent(Some(container.clone()));
    }
    unsafe {
        container.borrow_mut().children_mut().push(new_child.clone());
    }
}

pub fn remove_child_by_id(container: Rc<RefCell<Widget>>, id: String) {
    let position: Option<usize>;
    {
        position = container.borrow().children().iter().position(|x| x.borrow().id() == id);
    }
    if let Some(index) = position {
        unsafe {
            let removed_child = container.borrow_mut().children_mut().swap_remove(index).clone();
            removed_child.borrow_mut().set_parent(None);
        }
    }
}

pub fn remove_child_by_child(container: Rc<RefCell<Widget>>, child: Rc<RefCell<Widget>>) {
    let id: String;
    {
        id = child.borrow().id();
    }
    remove_child_by_id(container, id);
}

pub fn find_widget(container: Rc<RefCell<Widget>>, p: (u32, u32)) -> Option<Rc<RefCell<Widget>>> {
    let borrow_container = container.borrow();
    for child in &borrow_container.children() {
        let borrow_child = child.borrow();
        let new_p = (p.0 - borrow_container.pos().0, p.1 - borrow_container.pos().1);
        if borrow_child.visible() && borrow_child.contains(new_p) {
            return find_widget(child.clone(), new_p);
        }
    }

    return match borrow_container.contains(p) {
        true => Some(container.clone()),
        false => None
    }
}
