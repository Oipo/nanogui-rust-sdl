extern crate nanovg;

use std::cmp::max;
use widget::Widget;

#[derive(Copy, PartialEq, Clone)]
pub enum Alignment {
    Minimum = 0,
    Middle,
    Maximum,
    Fill
}

#[derive(Copy, PartialEq, Clone)]
pub enum Orientation {
    Horizontal = 0,
    Vertical
}

pub trait Layout {
    fn perform_layout(&self, &nanovg::Context, &Widget);
    fn preferred_size(&self, &nanovg::Context, &Widget) -> (u32, u32);
}

pub struct BoxLayout {
    pub alignment: Alignment,
    pub orientation: Orientation,
    pub margin: u32,
    pub spacing: u32
}

impl Layout for BoxLayout {
    fn perform_layout(&self, nanovg_context: &nanovg::Context, widget: &Widget) {
        let fs_w = widget.fixed_size();
        let mut container_size = [0u32, 0u32];
        let mut first = true;
        let axis1 = self.orientation as usize;
        let axis2 = (axis1 + 1)%2;
        let mut position = self.margin;

        if fs_w.0 > 0 {
            container_size[0] = fs_w.0;
        } else {
            container_size[0] = widget.size().0;
        }

        if fs_w.1 > 0 {
            container_size[1] = fs_w.1;
        } else {
            container_size[1] = widget.size().1;
        }

        if let Some(window) = widget.as_window() {
            if let Some(theme) = window.theme() {
                position += theme.borrow().window_header_height() - self.margin / 2;
            }
        }

        for child in &widget.children() {
            if !child.borrow().visible() {
                continue;
            }

            if first {
                first = false;
            } else {
                position += self.spacing;
            }

            let ps = child.borrow().preferred_size(nanovg_context);
            let fs = [child.borrow().fixed_size().0, child.borrow().fixed_size().1];
            let mut target_size = [0u32, 0u32];
            let mut pos = [0u32, 0u32];
            pos[axis1] = position;

            //println!("child {} ps {:?} fs {:?}", child.borrow().id(), ps, fs);

            if fs[0] > 0 {
                target_size[0] = fs[0];
            } else {
                target_size[0] = ps.0;
            }

            if fs[1] > 0 {
                target_size[1] = fs[1];
            } else {
                target_size[1] = ps.1;
            }

            match self.alignment {
                Alignment::Minimum => pos[axis2] = self.margin,
                Alignment::Middle => pos[axis2] = (container_size[axis2] - target_size[axis2]) / 2,
                Alignment::Maximum => pos[axis2] = container_size[axis2] - target_size[axis2] - self.margin,
                Alignment::Fill => {
                    pos[axis2] = self.margin;
                    if fs[axis2] > 0 {
                        target_size[axis2] = fs[axis2];
                    } else {
                        target_size[axis2] = container_size[axis2];
                    }
                }
            }

            child.borrow_mut().set_pos((pos[0], pos[1]));
            child.borrow_mut().set_size((target_size[0], target_size[1]));
            child.borrow().perform_layout(nanovg_context);
            position += target_size[axis1];
        }
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
