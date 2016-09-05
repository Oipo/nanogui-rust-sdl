extern crate nanovg;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
use widget::Widget;

#[derive(Copy, PartialEq, Clone)]
pub enum Alignment {
    Minimum,
    Middle,
    Maximum,
    Fill
}

#[derive(Copy, PartialEq, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical
}

pub trait Layout {
    fn perform_layout(&self, &nanovg::Context, Rc<RefCell<Widget>>);
    fn preferred_size(&self, &nanovg::Context, &Widget) -> (u32, u32);
}

pub struct BoxLayout {
    pub alignment: Alignment,
    pub orientation: Orientation,
    pub margin: u32,
    pub spacing: u32
}

impl Layout for BoxLayout {
    fn perform_layout(&self, nanovg_context: &nanovg::Context, widget: Rc<RefCell<Widget>>) {
        // TODO
    }

    fn preferred_size(&self, nanovg_context: &nanovg::Context, widget: &Widget) -> (u32, u32) {
        //println!("layout preferred size of {}", widget.id());
        let mut size = [self.margin*2, self.margin*2];

        if let Some(window) = widget.as_window() {
            if let Some(theme) = window.theme() {
                size[1] += theme.borrow().window_header_height() - self.margin / 2;
            }
        }

        let mut first = true;
        let axis1 = self.orientation as usize;
        let axis2 = (axis1 + 1)%2;

        for child in &widget.children() {
            if !child.borrow().visible() {
                continue;
            }

            //println!("child {} visible", child.borrow().id());

            if first {
                first = false;
            } else {
                size[axis1] += self.spacing;
            }

            let ps = child.borrow().preferred_size(nanovg_context);
            let fs = child.borrow().fixed_size();
            let mut target_size = [0u32, 0u32];

            //println!("child {} ps {:?} fs {:?}", child.borrow().id(), ps, fs);

            if fs.0 > 0 {
                target_size[0] = fs.0;
            } else {
                target_size[0] = ps.0;
            }

            if fs.1 > 0 {
                target_size[1] = fs.1;
            } else {
                target_size[1] = ps.1;
            }

            size[axis1] += target_size[axis1];
            size[axis2] = max(size[axis2], target_size[axis2] + 2*self.margin);
        }

        (size[0], size[1])
    }
}

impl BoxLayout {
    pub fn new(orientation: Orientation) -> BoxLayout {
        BoxLayout {
            orientation: orientation,
            alignment: Alignment::Middle,
            margin: 0,
            spacing: 0
        }
    }

    impl_get_set!(orientation, Orientation);
    impl_get_set!(alignment, Alignment);
    impl_get_set!(margin, u32);
    impl_get_set!(spacing, u32);
}
