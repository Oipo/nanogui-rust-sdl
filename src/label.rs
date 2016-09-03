extern crate sdl2;
extern crate nanovg;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use widget::{Widget, WidgetObj};
use theme::Theme;

#[allow(dead_code)]
pub struct LabelObj {
    widget: WidgetObj,
    caption: String,
    font: String,
    nanovg_font: nanovg::Font,
    color: (f32, f32, f32, f32)
}

#[allow(unused_variables)]
impl Widget for LabelObj {
    fn parent(&self) -> Option<&Weak<RefCell<Widget>>> {
        self.widget.parent.as_ref()
    }

    unsafe fn set_parent(&mut self, parent: Option<Rc<RefCell<Widget>>>) {
        self.widget.parent = match parent {
            Some(val) => { Some(Rc::downgrade(&val)) },
            None => None
        }
    }

    fn children(&self) -> Vec<Rc<RefCell<Widget>>> {
        self.widget.children.clone()
    }

    unsafe fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<Widget>>> {
        &mut self.widget.children
    }

    fn id(&self) -> String {
        self.widget.id.clone()
    }

    fn pos(&self) -> (u32, u32) {
        self.widget.pos
    }

    fn set_pos(&mut self, p: (u32, u32)) {
        self.widget.pos = p;
    }

    fn size(&self) -> (u32, u32) {
        self.widget.size
    }

    fn set_size(&mut self, s: (u32, u32)) {
        self.widget.size = s;
    }

    fn fixed_size(&self) -> (u32, u32) {
        self.widget.fixed_size
    }

    fn set_fixed_size(&mut self, s: (u32, u32)) {
        self.widget.fixed_size = s;
    }

    fn font_size(&self) -> i32 {
        self.widget.font_size
    }

    fn set_font_size(&mut self, s: i32) {
        self.widget.font_size = s;
    }

    fn theme(&self) -> Option<&Rc<RefCell<Theme>>> {
        self.widget.theme.as_ref()
    }

    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>) {
        self.widget.theme = theme;
    }

    fn enabled(&self) -> bool {
        self.widget.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.widget.enabled = enabled;
    }

    fn tooltip(&self) -> String {
        self.widget.tooltip.clone()
    }

    fn set_tooltip(&mut self, tooltip: String) {
        self.widget.tooltip = tooltip;
    }

    fn visible(&self) -> bool {
        self.widget.visible
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        for child in &self.widget.children {
            let p_mut = child.borrow_mut();
            p_mut.draw(nanovg_context);
        }

        /*println!("drawing label: {} - {} - {} {} {} {} - @{} {}",
            self.font,
            self.widget.font_size(),
            self.color.0, self.color.1, self.color.2, self.color.3,
            self.widget.pos().0, self.widget.pos().1);*/

        nanovg_context.font_face(&self.font);
        nanovg_context.font_size(self.widget.font_size as f32);
        nanovg_context.fill_color(nanovg::Color::rgba_f(self.color.0 as f32, self.color.1 as f32, self.color.2 as f32, self.color.3 as f32));
        if self.widget.fixed_size.0 > 0 {
            nanovg_context.text_align(nanovg::LEFT | nanovg::TOP);
            nanovg_context.text_box(self.widget.pos.0 as f32, self.widget.pos.1 as f32, self.widget.fixed_size.0 as f32, &self.caption);
        } else {
            nanovg_context.text_align(nanovg::LEFT | nanovg::TOP);
            nanovg_context.text_box(self.widget.pos.0 as f32, self.widget.pos.1 as f32 + self.widget.size.1 as f32 * 0.5, self.widget.fixed_size.0 as f32, &self.caption);
        }
    }

    fn absolute_position(&self) -> (u32, u32) {
        if let Some(ref val) = self.widget.parent {
            if let Some(ref val_upgraded) = val.upgrade() {
                let (par_x, par_y) = val_upgraded.borrow().absolute_position();
                return (par_x + self.widget.pos.0, par_y + self.widget.pos.1)
            }
        }

        return self.widget.pos.clone();
    }

    fn visible_recursive(&self) -> bool {
        if !self.widget.visible {
            return false
        }

        if let Some(ref val) = self.widget.parent {
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

impl LabelObj {
    pub fn new_create_font(id: String, caption: String, font_filename: String, nanovg_context: &nanovg::Context) -> Rc<RefCell<LabelObj>> {
        Rc::new(RefCell::new(LabelObj {
            widget: WidgetObj::new(id),
            caption: caption,
            color: (255.0, 255.0, 255.0, 125.0),
            font: font_filename.clone(),
            nanovg_font: nanovg_context.create_font(&font_filename, &font_filename).unwrap()
        }))
    }

    pub fn new(id: String, caption: String, font: nanovg::Font) -> Rc<RefCell<LabelObj>> {
        Rc::new(RefCell::new(LabelObj {
            widget: WidgetObj::new(id),
            caption: caption,
            color: (255.0, 255.0, 255.0, 125.0),
            font: "".to_string(),
            nanovg_font: font
        }))
    }
}
