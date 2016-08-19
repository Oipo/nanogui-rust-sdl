extern crate nanovg;

use std::fmt;
//use std::mem;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
//use std::ops::{Deref, DerefMut};

pub struct WidgetObjRef(pub Rc<RefCell<WidgetObj>>);

#[derive(Debug)]
pub struct WidgetObj {
    parent: Option<Weak<RefCell<Widget>>>,
    children: Vec<Rc<RefCell<Widget>>>,
    //theme: theme,
    //layout: layout,
    id: String,
    pos: (u32, u32),
    size: (u32, u32),
    fixed_size: (u32, u32),
    visible: bool,
    enabled: bool,
    focused: bool,
    mouse_focus: bool,
    tooltip: String,
    font_size: u32,
    //cursor: cursor
}

pub trait Widget {
    fn widget_obj(&self) -> &WidgetObj;
    fn widget_obj_mut(&mut self) -> &mut WidgetObj;

    fn parent(&self) -> &Option<Weak<RefCell<Widget>>> {
        &self.widget_obj().parent
    }

    fn children(&self) -> &Vec<Rc<RefCell<Widget>>> {
        &self.widget_obj().children
    }

    fn remove_child_by_id(&mut self, id: String) -> Option<Rc<RefCell<Widget>>> {
        let ref mut temp_children = self.widget_obj_mut().children;
        let position = temp_children.iter().position(|x| x.borrow().id() == id);
        match position {
            Some(index) => {
                let removed_child = temp_children.swap_remove(index).clone();
                removed_child.borrow_mut().widget_obj_mut().parent = None;
                Some(removed_child)
            },
            _ => None
        }
    }

    fn remove_child_by_child(&mut self, child: &mut Widget) {
        let ref mut temp_children = self.widget_obj_mut().children;
        let position = temp_children.iter().position(|x| x.borrow().id() == child.id());
        match position {
            Some(index) => {
                temp_children.swap_remove(index).clone();
                child.widget_obj_mut().parent = None;
            },
            _ => {}
        }
    }

    fn get_child_by_id(&self, id: String) -> Option<Rc<RefCell<Widget>>> {
        match self.widget_obj().children.iter().position(|x| x.borrow().id() == id) {
            Some(index) => Some(self.widget_obj().children.get(index).unwrap().clone()),
            _ => None
        }
    }

    fn id(&self) -> String {
        self.widget_obj().id.clone()
    }

    fn pos(&self) -> (u32, u32) {
        self.widget_obj().pos
    }

    fn set_pos(&mut self, p: (u32, u32)) {
        self.widget_obj_mut().pos = p;
    }

    fn size(&self) -> (u32, u32) {
        self.widget_obj().size
    }

    fn set_size(&mut self, s: (u32, u32)) {
        self.widget_obj_mut().size = s;
    }

    fn fixed_size(&self) -> (u32, u32) {
        self.widget_obj().fixed_size
    }

    fn set_fixed_size(&mut self, s: (u32, u32)) {
        self.widget_obj_mut().fixed_size = s;
    }

    fn font_size(&self) -> u32 {
        self.widget_obj().font_size
    }

    fn set_font_size(&mut self, s: u32) {
        self.widget_obj_mut().font_size = s;
    }

    fn visible(&self) -> bool {
        self.widget_obj().visible
    }

    fn absolute_position(&self) -> (u32, u32) {
        match *self.parent() {
            None => self.pos().clone(),
            Some(ref val) =>  {
                let parent = val.upgrade().unwrap();
                let (par_x, par_y) = parent.borrow().absolute_position();
                (par_x + self.pos().0, par_y + self.pos().1)
            }
        }
    }

    fn visible_recursive(&self) -> bool {
        if !self.visible() {
            return false
        }

        match *self.parent() {
            None => self.visible(),
            Some(ref val) => {
                let parent = val.upgrade().unwrap();
                let visible = parent.borrow().visible_recursive();
                visible
            }
        }
    }

    fn contains(&self, p: (u32, u32)) -> bool {
        // TODO
        return false
    }

    fn find_widget(&self, p: (u32, u32)) -> Option<Box<Widget>> {
        // TODO
        return None
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        for child in &self.widget_obj().children {
            child.borrow().draw(nanovg_context);
        }
    }
}

impl PartialEq for Widget {
    fn eq(&self, other: &Widget) -> bool {
        self.id() == other.id()
    }
}

impl fmt::Debug for Widget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Widget {} {{ x: {}, y: {} }}", self.id(), self.pos().0, self.pos().1)
    }
}

impl Widget for WidgetObj {
    fn widget_obj(&self) -> &WidgetObj {
        self
    }

    fn widget_obj_mut(&mut self) -> &mut WidgetObj {
        self
    }
}

impl WidgetObjRef {
    pub fn new(id: String) -> WidgetObjRef {
        WidgetObjRef(Rc::new(RefCell::new(WidgetObj {
            parent: None,
            children: Vec::new(),
            id: id,
            pos: (0, 0),
            size: (0, 0),
            fixed_size: (0, 0),
            visible: true,
            enabled: true,
            focused: false,
            mouse_focus: false,
            tooltip: String::new(),
            font_size: 12
        })))
    }

    pub fn push_child(&self, new_child: Rc<RefCell<WidgetObj>>) {
        {
            let child_id: String;
            let mut parent: Option<Weak<RefCell<Widget>>> = None;
            {
                let new_child_borrow = new_child.borrow();
                child_id = new_child_borrow.id().clone();
                match new_child_borrow.parent {
                    Some(ref val) => parent = Some(val.clone()),
                    _ => {}
                }
            }

            if self.0.borrow().id() == child_id {
                panic!("Do not add a widget to itself.");
            }

            match parent {
                Some(ref val) => {
                    let upgraded_parent = match val.upgrade() {
                        Some(val) => val,
                        None => {panic!("Uh-oh");}
                    };
                    upgraded_parent.borrow_mut().remove_child_by_id(child_id);
                },
                _ => {}
            }
        }
        {
            new_child.borrow_mut().parent = Some(Rc::downgrade(&(self.0.clone() as Rc<RefCell<Widget>>)));
        }
        self.0.borrow_mut().children.push(new_child);
    }

    // === Widget implementation ==
    pub fn parent(&self) -> Option<Weak<RefCell<Widget>>> {
        let mut parent: Option<Weak<RefCell<Widget>>> = None;
        {
            let obj_borrow = self.0.borrow();
            match *obj_borrow.parent() {
                Some(ref val) => parent = Some(val.clone()),
                _ => {}
            }
        }
        return parent;
    }

    pub fn children(&self) -> Vec<Rc<RefCell<Widget>>> {
        let children: Vec<Rc<RefCell<Widget>>>;
        {
            let obj_borrow = self.0.borrow();
            children = obj_borrow.children.clone();
        }
        return children;
    }

    pub fn remove_child_by_id(&self, id: String) -> Option<Rc<RefCell<Widget>>> {
        self.0.borrow_mut().remove_child_by_id(id)
    }

    pub fn remove_child_by_child(&self, child: &mut Widget) {
        self.0.borrow_mut().remove_child_by_child(child)
    }

    pub fn get_child_by_id(&self, id: String) -> Option<Rc<RefCell<Widget>>> {
        self.0.borrow().get_child_by_id(id)
    }

    pub fn id(&self) -> String {
        self.0.borrow().id.clone()
    }

    pub fn pos(&self) -> (u32, u32) {
        self.0.borrow().pos
    }

    pub fn set_pos(&self, p: (u32, u32)) {
        self.0.borrow_mut().pos = p;
    }

    pub fn size(&self) -> (u32, u32) {
        self.0.borrow().size
    }

    pub fn set_size(&self, s: (u32, u32)) {
        self.0.borrow_mut().size = s;
    }

    pub fn fixed_size(&self) -> (u32, u32) {
        self.0.borrow().fixed_size
    }

    pub fn set_fixed_size(&self, s: (u32, u32)) {
        self.0.borrow_mut().fixed_size = s;
    }

    pub fn font_size(&self) -> u32 {
        self.0.borrow().font_size
    }

    pub fn set_font_size(&self, s: u32) {
        self.0.borrow_mut().font_size = s;
    }

    pub fn visible(&self) -> bool {
        self.0.borrow().visible
    }
}
