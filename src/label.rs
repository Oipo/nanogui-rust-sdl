extern crate sdl2;
extern crate nanovg;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use widget::{Widget, WidgetObj, WidgetObjRef};

pub struct LabelObj {
    widget: WidgetObjRef,
    caption: String,
    font: String,
    nanovg_font: nanovg::Font,
    color: (f32, f32, f32, f32)
}

impl Widget for LabelObj {
    fn widget_obj(&self) -> &WidgetObj {
        &self.widget.0
    }

    fn widget_obj_mut(&mut self) -> &mut WidgetObj {
        &mut self.widget.0
    }

    /*fn widget_obj_ref(&self) -> &WidgetObjRef {
        &self.widget
    }

    fn widget_obj_ref_mut(&mut self) -> &mut WidgetObjRef {
        &mut self.widget
    }*/

    fn draw(&self, nanovg_context: &nanovg::Context) {
        self.widget.draw(nanovg_context);

        /*println!("darwin label: {} - {} - {} {} {} {} - @{} {}",
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
}

impl LabelObj {
    pub fn new(id: String, caption: String, font_filename: String, nanovg_context: &nanovg::Context) -> Rc<RefCell<LabelObj>> {
        Rc::new(RefCell::new(LabelObj {
            widget: WidgetObj::new(id),
            caption: caption,
            color: (255.0, 255.0, 255.0, 125.0),
            font: font_filename.clone(),
            nanovg_font: nanovg_context.create_font(&font_filename, &font_filename).unwrap()
        }))
    }
}
