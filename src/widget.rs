extern crate nanovg;

use std::fmt;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use theme::Theme;

#[allow(dead_code)]
pub struct WidgetObj {
    pub parent: Option<Weak<RefCell<Widget>>>,
    pub children: Vec<Rc<RefCell<Widget>>>,
    pub theme: Option<Rc<RefCell<Theme>>>,
    //pub layout: layout,
    pub id: String,
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub fixed_size: (u32, u32),
    pub visible: bool,
    pub enabled: bool,
    pub focused: bool,
    pub mouse_focus: bool,
    pub tooltip: String,
    pub font_size: i32,
    //pub cursor: cursor
}

pub trait Widget {
    fn parent(&self) -> Option<&Weak<RefCell<Widget>>>;
    unsafe fn set_parent(&mut self, Option<Rc<RefCell<Widget>>>);
    fn children(&self) -> Vec<Rc<RefCell<Widget>>>;
    unsafe fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<Widget>>>;
    fn id(&self) -> String;
    fn pos(&self) -> (u32, u32);
    fn set_pos(&mut self, p: (u32, u32));
    fn size(&self) -> (u32, u32);
    fn set_size(&mut self, s: (u32, u32));
    fn fixed_size(&self) -> (u32, u32);
    fn set_fixed_size(&mut self, s: (u32, u32));
    fn font_size(&self) -> i32;
    fn set_font_size(&mut self, s: i32);
    fn theme(&self) -> Option<&Rc<RefCell<Theme>>>;
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
    fn parent(&self) -> Option<&Weak<RefCell<Widget>>> {
        self.parent.as_ref()
    }

    unsafe fn set_parent(&mut self, parent: Option<Rc<RefCell<Widget>>>) {
        self.parent = match parent {
            Some(val) => { Some(Rc::downgrade(&val)) },
            None => None
        }
    }

    fn children(&self) -> Vec<Rc<RefCell<Widget>>> {
        self.children.clone()
    }

    unsafe fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<Widget>>> {
        &mut self.children
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
        self.font_size
    }

    fn set_font_size(&mut self, s: i32) {
        self.font_size = s;
    }

    fn theme(&self) -> Option<&Rc<RefCell<Theme>>> {
        self.theme.as_ref()
    }

    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>) {
        self.theme = theme;
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
        self.tooltip = tooltip;
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn absolute_position(&self) -> (u32, u32) {
        if let Some(ref val) = self.parent {
            if let Some(ref val_upgraded) = val.upgrade() {
                let (par_x, par_y) = val_upgraded.borrow().absolute_position();
                return (par_x + self.pos.0, par_y + self.pos.1)
            }
        }

        return self.pos.clone();
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        for child in &self.children {
            let p_mut = child.borrow_mut();
            p_mut.draw(nanovg_context);
        }
    }

    fn visible_recursive(&self) -> bool {
        if !self.visible {
            return false
        }

        if let Some(ref val) = self.parent {
            if let Some(ref val_upgraded) = val.upgrade() {
                return val_upgraded.borrow().visible_recursive();
            }
        }

        return true;
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

impl Drop for WidgetObj {
    fn drop(&mut self) {
        println!("dropping widgetobj {}", self.id);
        while let Some(child) = self.children.pop() {
            unsafe {
                println!("address child: {:p}", child);
                println!("unlinking child {}", child.borrow().id());
                child.borrow_mut().set_parent(None);
            }
        }
    }
}

impl WidgetObj {
    pub fn new(id: String) -> WidgetObj {
        WidgetObj {
            parent: None,
            children: Vec::new(),
            theme: None,
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
        }
    }
}
