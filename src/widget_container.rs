use std::rc::{Rc, Weak};
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
    let mut position: Option<usize> = None;
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
    let mut id: String = "".to_string();
    {
        id = child.borrow().id();
    }
    remove_child_by_id(container, id);
}
