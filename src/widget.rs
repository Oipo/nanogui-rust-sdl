extern crate nanovg;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;
use theme::Theme;

pub struct WidgetObjRef(pub Rc<RefCell<WidgetObj>>);

#[allow(dead_code)]
pub struct WidgetObj {
    parent: Option<*mut Widget>,
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
    fn parent(&self) -> Option<*mut Widget>;
    fn set_parent(&mut self, Option<*mut Widget>);
    fn children(&self) -> Vec<Rc<RefCell<Widget>>>;
    fn remove_child_by_id(&mut self, id: String) -> Option<Rc<RefCell<Widget>>>;
    fn remove_child_by_child(&mut self, child: &mut Widget);
    fn get_child_by_id(&self, id: String) -> Option<Rc<RefCell<Widget>>>;
    fn push_child(&mut self, new_child: Rc<RefCell<Widget>>);
    fn id(&self) -> String;
    fn pos(&self) -> (u32, u32);
    fn set_pos(&mut self, p: (u32, u32));
    fn size(&self) -> (u32, u32);
    fn set_size(&mut self, s: (u32, u32));
    fn fixed_size(&self) -> (u32, u32);
    fn set_fixed_size(&mut self, s: (u32, u32));
    fn font_size(&self) -> i32;
    fn set_font_size(&mut self, s: i32);
    fn theme(&self) -> Option<Rc<RefCell<Theme>>>;
    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>);
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn tooltip(&self) -> String;
    fn set_tooltip(&mut self, tooltip: String);
    fn visible(&self) -> bool;
    fn absolute_position(&self) -> (u32, u32);
    fn visible_recursive(&self) -> bool;
    fn contains(&self, p: (u32, u32)) -> bool;
    fn find_widget(&self, p: (u32, u32)) -> Option<Box<Widget>>;
    fn draw(&self, nanovg_context: &nanovg::Context);
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

#[allow(unused_variables)]
impl Widget for WidgetObj {
    fn parent(&self) -> Option<*mut Widget> {
        match self.parent {
            Some(ref val) => Some(val.clone()),
            None => None
        }
    }

    fn set_parent(&mut self, parent: Option<*mut Widget>) {
        self.parent = parent;
    }

    fn children(&self) -> Vec<Rc<RefCell<Widget>>> {
        self.children.clone()
    }

    fn remove_child_by_id(&mut self, id: String) -> Option<Rc<RefCell<Widget>>> {
        let position = self.children.iter().position(|x| x.borrow().id() == id);
        match position {
            Some(index) => {
                let removed_child = self.children.swap_remove(index).clone();
                removed_child.borrow_mut().set_parent(None);
                Some(removed_child)
            },
            _ => None
        }
    }

    fn remove_child_by_child(&mut self, child: &mut Widget) {
        let position = self.children.iter().position(|x| x.borrow().id() == child.id());
        match position {
            Some(index) => {
                self.children.swap_remove(index).clone();
                child.set_parent(None);
            },
            _ => {}
        }
    }

    fn get_child_by_id(&self, id: String) -> Option<Rc<RefCell<Widget>>> {
        match self.children.iter().position(|x| x.borrow().id() == id) {
            Some(index) => Some(self.children.get(index).unwrap().clone()),
            _ => None
        }
    }

    fn push_child(&mut self, new_child: Rc<RefCell<Widget>>) {
        panic!("push_child will never work on WidgetObj");
    }

    fn id(&self) -> String {
        self.id.clone()
    }

    fn pos(&self) -> (u32, u32) {
        self.pos
    }

    fn set_pos(&mut self, p: (u32, u32)) {
        self.pos = p;
    }

    fn size(&self) -> (u32, u32) {
        self.size
    }

    fn set_size(&mut self, s: (u32, u32)) {
        self.size = s;
    }

    fn fixed_size(&self) -> (u32, u32) {
        self.fixed_size
    }

    fn set_fixed_size(&mut self, s: (u32, u32)) {
        self.fixed_size = s;
    }

    fn font_size(&self) -> i32 {
        match self.theme {
            Some(ref val) => {
                if self.font_size < 0 {
                    return val.borrow().standard_font_size();
                }

                return self.font_size;
            },
            None => self.font_size
        }
    }

    fn set_font_size(&mut self, s: i32) {
        self.font_size = s;
    }

    fn theme(&self) -> Option<Rc<RefCell<Theme>>> {
        match self.theme {
            Some(ref val) => Some(val.clone()),
            None => None
        }
    }

    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>) {
        self.theme = theme
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    fn tooltip(&self) -> String {
        self.tooltip.clone()
    }

    fn set_tooltip(&mut self, tooltip: String) {
        self.tooltip = tooltip.clone();
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn absolute_position(&self) -> (u32, u32) {
        match self.parent {
            Some(val) =>  {
                unsafe {
                    let (par_x, par_y) = (*val).absolute_position();
                    (par_x + self.pos().0, par_y + self.pos().1)
                }
            },
            None => self.pos.clone()
        }
    }

    fn visible_recursive(&self) -> bool {
        if !self.visible {
            return false
        }

        match self.parent{
            Some(val) => unsafe { (*val).visible_recursive() },
            None => self.visible
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
        for child in &self.children {
            child.borrow().draw(nanovg_context);
        }
    }
}

#[allow(unused_variables)]
impl Widget for WidgetObjRef {
    fn parent(&self) -> Option<*mut Widget> {
        match self.0.borrow().parent() {
            Some(val) => Some(val),
            None => None
        }
    }

    fn set_parent(&mut self, parent: Option<*mut Widget>) {
        self.0.borrow_mut().set_parent(parent);
    }

    fn children(&self) -> Vec<Rc<RefCell<Widget>>> {
        let children: Vec<Rc<RefCell<Widget>>>;
        {
            let obj_borrow = self.0.borrow();
            children = obj_borrow.children.clone();
        }
        return children;
    }

    fn remove_child_by_id(&mut self, id: String) -> Option<Rc<RefCell<Widget>>> {
        self.0.borrow_mut().remove_child_by_id(id)
    }

    fn remove_child_by_child(&mut self, child: &mut Widget) {
        self.0.borrow_mut().remove_child_by_child(child)
    }

    fn push_child(&mut self, new_child: Rc<RefCell<Widget>>) {
        {
            let child_id: String;
            let mut parent: Option<*mut Widget> = None;
            {
                let new_child_borrow = new_child.borrow();
                child_id = new_child_borrow.id().clone();
                match new_child_borrow.parent() {
                    Some(val) => parent = Some(val),
                    _ => {}
                }
            }

            if self.0.borrow().id() == child_id {
                panic!("Do not add a widget to itself.");
            }

            match parent {
                Some(val) => { unsafe { (*val).remove_child_by_id(child_id); } },
                None => {}
            }
        }
        {
            new_child.borrow_mut().set_parent(Some(&mut *self));
        }
        self.0.borrow_mut().children.push(new_child);
    }

    fn get_child_by_id(&self, id: String) -> Option<Rc<RefCell<Widget>>> {
        self.0.borrow().get_child_by_id(id)
    }

    fn id(&self) -> String {
        self.0.borrow().id.clone()
    }

    fn pos(&self) -> (u32, u32) {
        self.0.borrow().pos
    }

    fn set_pos(&mut self, p: (u32, u32)) {
        self.0.borrow_mut().pos = p;
    }

    fn size(&self) -> (u32, u32) {
        self.0.borrow().size
    }

    fn set_size(&mut self, s: (u32, u32)) {
        self.0.borrow_mut().size = s;
    }

    fn fixed_size(&self) -> (u32, u32) {
        self.0.borrow().fixed_size
    }

    fn set_fixed_size(&mut self, s: (u32, u32)) {
        self.0.borrow_mut().fixed_size = s;
    }

    fn font_size(&self) -> i32 {
        self.0.borrow().font_size
    }

    fn set_font_size(&mut self, s: i32) {
        self.0.borrow_mut().font_size = s;
    }

    fn theme(&self) -> Option<Rc<RefCell<Theme>>> {
        let mut theme: Option<Rc<RefCell<Theme>>> = None;
        {
            let obj_borrow = self.0.borrow();
            match obj_borrow.theme() {
                Some(ref val) => theme = Some(val.clone()),
                _ => {}
            }
        }
        return theme;
    }

    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>) {
        self.0.borrow_mut().theme = theme;
    }

    fn enabled(&self) -> bool {
        self.0.borrow().enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.0.borrow_mut().enabled = enabled;
    }

    fn tooltip(&self) -> String {
        self.0.borrow().tooltip.clone()
    }

    fn set_tooltip(&mut self, tooltip: String) {
        self.0.borrow_mut().tooltip = tooltip.clone();
    }

    fn visible(&self) -> bool {
        self.0.borrow().visible
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        self.0.borrow().draw(nanovg_context);
    }

    fn absolute_position(&self) -> (u32, u32) {
        match self.parent() {
            Some(val) =>  {
                unsafe {
                    let (par_x, par_y) = (*val).absolute_position();
                    (par_x + self.pos().0, par_y + self.pos().1)
                }
            },
            None => self.pos().clone()
        }
    }

    fn visible_recursive(&self) -> bool {
        if !self.visible() {
            return false
        }

        match self.parent() {
            Some(val) => unsafe { (*val).visible_recursive() },
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
}

impl Drop for WidgetObjRef {
    fn drop(&mut self) {
        println!("dropping widgetobjref {}", self.0.borrow().id());
        //drop(self.0);
    }
}

impl Drop for WidgetObj {
    fn drop(&mut self) {
        println!("dropping widgetobj {}", self.id);
        for child in &self.children {
            {
                println!("setting child {} parent to None", child.borrow().id());
            }
            child.borrow_mut().set_parent(None)
        }
    }
}

impl WidgetObjRef {
    pub fn new(id: String, parent: Option<Rc<RefCell<Widget>>>) -> WidgetObjRef {

        let mut parent_pointer: Option<*mut Widget> = None;
        let mut parent_theme: Option<Rc<RefCell<Theme>>> = None;
        match parent {
            Some(ref val) => {
                match val.borrow().theme() {
                    Some(ref val) => { parent_theme = Some(val.clone()); },
                    None => {}
                };
                parent_pointer = Some(val.borrow_mut().deref_mut() as *mut Widget);
            },
            None => {}
        };

        WidgetObjRef(Rc::new(RefCell::new(WidgetObj {
            parent: parent_pointer,
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
}
