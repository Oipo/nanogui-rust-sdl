extern crate sdl2;
extern crate nanovg;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use widget::{Widget, WidgetObj, WidgetObjRef};
use theme::Theme;

#[allow(dead_code)]
pub struct LabelObj {
    widget: WidgetObjRef,
    caption: String,
    font: String,
    nanovg_font: nanovg::Font,
    color: (f32, f32, f32, f32)
}

#[allow(unused_variables)]
impl Widget for LabelObj {
    fn parent(&self) -> Option<*mut Widget> {
        self.widget.parent()
    }

    fn set_parent(&mut self, parent: Option<*mut Widget>) {
        self.widget.set_parent(parent);
    }

    fn children(&self) -> Vec<Rc<RefCell<Widget>>> {
        self.widget.children()
    }

    fn remove_child_by_id(&mut self, id: String) -> Option<Rc<RefCell<Widget>>> {
        self.widget.remove_child_by_id(id)
    }

    fn remove_child_by_child(&mut self, child: &mut Widget) {
        self.widget.remove_child_by_child(child);
    }

    fn push_child(&mut self, new_child: Rc<RefCell<Widget>>) {
        self.widget.push_child(new_child);
    }

    fn get_child_by_id(&self, id: String) -> Option<Rc<RefCell<Widget>>> {
        self.widget.get_child_by_id(id)
    }

    fn id(&self) -> String {
        self.widget.id()
    }

    fn pos(&self) -> (u32, u32) {
        self.widget.pos()
    }

    fn set_pos(&mut self, p: (u32, u32)) {
        self.widget.set_pos(p);
    }

    fn size(&self) -> (u32, u32) {
        self.widget.size()
    }

    fn set_size(&mut self, s: (u32, u32)) {
        self.widget.set_size(s);
    }

    fn fixed_size(&self) -> (u32, u32) {
        self.widget.fixed_size()
    }

    fn set_fixed_size(&mut self, s: (u32, u32)) {
        self.widget.set_fixed_size(s);
    }

    fn font_size(&self) -> i32 {
        self.widget.font_size()
    }

    fn set_font_size(&mut self, s: i32) {
        self.widget.set_font_size(s);
    }

    fn theme(&self) -> Option<Rc<RefCell<Theme>>> {
        self.widget.theme()
    }

    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>) {
        self.widget.set_theme(theme);
    }

    fn enabled(&self) -> bool {
        self.widget.enabled()
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.widget.set_enabled(enabled);
    }

    fn tooltip(&self) -> String {
        self.widget.tooltip()
    }

    fn set_tooltip(&mut self, tooltip: String) {
        self.widget.set_tooltip(tooltip);
    }

    fn visible(&self) -> bool {
        self.widget.visible()
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        self.widget.draw(nanovg_context);

        /*println!("drawing label: {} - {} - {} {} {} {} - @{} {}",
            self.font,
            self.widget.font_size(),
            self.color.0, self.color.1, self.color.2, self.color.3,
            self.widget.pos().0, self.widget.pos().1);*/

        nanovg_context.font_face(&self.font);
        nanovg_context.font_size(self.widget.font_size() as f32);
        nanovg_context.fill_color(nanovg::Color::rgba_f(self.color.0 as f32, self.color.1 as f32, self.color.2 as f32, self.color.3 as f32));
        if self.widget.fixed_size().0 > 0 {
            nanovg_context.text_align(nanovg::LEFT | nanovg::TOP);
            nanovg_context.text_box(self.widget.pos().0 as f32, self.widget.pos().1 as f32, self.widget.fixed_size().0 as f32, &self.caption);
        } else {
            nanovg_context.text_align(nanovg::LEFT | nanovg::TOP);
            nanovg_context.text_box(self.widget.pos().0 as f32, self.widget.pos().1 as f32 + self.widget.size().1 as f32 * 0.5, self.widget.fixed_size().0 as f32, &self.caption);
        }
    }

    fn absolute_position(&self) -> (u32, u32) {
        self.widget.absolute_position()
    }

    fn visible_recursive(&self) -> bool {
        self.widget.visible_recursive()
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
    pub fn new_create_font(id: String, caption: String, font_filename: String, nanovg_context: &nanovg::Context) -> LabelObj {
        LabelObj {
            widget: WidgetObjRef::new(id, None),
            caption: caption,
            color: (255.0, 255.0, 255.0, 125.0),
            font: font_filename.clone(),
            nanovg_font: nanovg_context.create_font(&font_filename, &font_filename).unwrap()
        }
    }

    pub fn new(id: String, caption: String, font: nanovg::Font, nanovg_context: &nanovg::Context) -> LabelObj {
        LabelObj {
            widget: WidgetObjRef::new(id, None),
            caption: caption,
            color: (255.0, 255.0, 255.0, 125.0),
            font: "".to_string(),
            nanovg_font: font
        }
    }
}
