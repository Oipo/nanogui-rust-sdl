extern crate nanovg;

use std::fmt;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use theme::Theme;

pub struct WidgetObjRef(pub Rc<RefCell<WidgetObj>>);

pub struct WidgetObj {
    parent: Option<Weak<RefCell<Widget>>>,
    children: Vec<Rc<RefCell<Widget>>>,
    theme: Option<Rc<RefCell<Theme>>>,
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
    font_size: i32,
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

    fn font_size(&self) -> i32 {
        match self.widget_obj().theme {
            Some(ref val) => {
                if self.widget_obj().font_size < 0 {
                    return val.borrow().standard_font_size();
                }

                return self.widget_obj().font_size;
            },
            None => self.widget_obj().font_size
        }
    }

    fn set_font_size(&mut self, s: i32) {
        self.widget_obj_mut().font_size = s;
    }

    fn theme(&self) -> &Option<Rc<RefCell<Theme>>> {
        &self.widget_obj().theme
    }

    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>) {
        self.widget_obj_mut().theme = theme
    }

    fn enabled(&self) -> bool {
        self.widget_obj().enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.widget_obj_mut().enabled = enabled;
    }

    fn tooltip(&self) -> String {
        self.widget_obj().tooltip.clone()
    }

    fn set_tooltip(&mut self, tooltip: String) {
        self.widget_obj_mut().tooltip = tooltip.clone();
    }

    fn visible(&self) -> bool {
        self.widget_obj().visible
    }

    fn absolute_position(&self) -> (u32, u32) {
        match *self.parent() {
            Some(ref val) =>  {
                let parent = val.upgrade().unwrap();
                let (par_x, par_y) = parent.borrow().absolute_position();
                (par_x + self.pos().0, par_y + self.pos().1)
            },
            None => self.pos().clone()
        }
    }

    fn visible_recursive(&self) -> bool {
        if !self.visible() {
            return false
        }

        match *self.parent() {
            Some(ref val) => {
                let parent = val.upgrade().unwrap();
                let visible = parent.borrow().visible_recursive();
                visible
            },
            None => self.visible()
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
    pub fn new(id: String, parent: Option<Rc<RefCell<Widget>>>) -> WidgetObjRef {
        let parent_theme: Option<Rc<RefCell<Theme>>> = match parent {
            Some(ref val) => {
                match *val.borrow().theme() {
                    Some(ref val) => Some(val.clone()),
                    None => None
                }
            },
            None => None
        };
        let parent_weak: Option<Weak<RefCell<Widget>>> = match parent {
            Some(ref val) => Some(Rc::downgrade(val)),
            None => None
        };
        WidgetObjRef(Rc::new(RefCell::new(WidgetObj {
            parent: parent_weak,
            children: Vec::new(),
            theme: parent_theme,
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

    pub fn font_size(&self) -> i32 {
        self.0.borrow().font_size
    }

    pub fn set_font_size(&self, s: i32) {
        self.0.borrow_mut().font_size = s;
    }

    pub fn theme(&self) -> Option<Rc<RefCell<Theme>>> {
        let mut theme: Option<Rc<RefCell<Theme>>> = None;
        {
            let obj_borrow = self.0.borrow();
            match *obj_borrow.theme() {
                Some(ref val) => theme = Some(val.clone()),
                _ => {}
            }
        }
        return theme;
    }

    pub fn set_theme(&self, theme: Option<Rc<RefCell<Theme>>>) {
        self.0.borrow_mut().theme = theme;
    }

    pub fn enabled(&self) -> bool {
        self.0.borrow().enabled
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.0.borrow_mut().enabled = enabled;
    }

    pub fn tooltip(&self) -> String {
        self.0.borrow().tooltip.clone()
    }

    pub fn set_tooltip(&self, tooltip: String) {
        self.0.borrow_mut().tooltip = tooltip.clone();
    }

    pub fn visible(&self) -> bool {
        self.0.borrow().visible
    }
}
