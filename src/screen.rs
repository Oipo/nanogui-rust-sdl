extern crate sdl2;
extern crate sdl2_sys;
extern crate nanovg;

use std::cell::RefCell;
use std::rc::Rc;
use widget::{Widget, WidgetObj, WidgetObjRef};
use theme::Theme;

#[allow(dead_code)]
pub struct ScreenObj {
    widget: WidgetObjRef,
    nanovg_context: nanovg::Context,
    //focuspath?
    framebuffer_size: (u32, u32),
    pixel_ratio: f32,
    mouse_state: i32,
    modifiers: i32,
    mouse_pos: (u32, u32),
    drag_active: bool,
    drag_widget: Option<WidgetObj>,
    last_interaction: u32,
    process_events: bool,
    background: (f32, f32, f32),
    caption: String
}

#[allow(unused_variables)]
impl Widget for ScreenObj {
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
        self.draw_widgets()
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

impl ScreenObj {
    pub fn new(id: String, caption: String, window: &mut sdl2::video::WindowRef) -> ScreenObj {

        let winsize = window.size();
        window.set_title(&caption);

        unsafe {
            let mut screen: ScreenObj = ScreenObj {
                widget: WidgetObjRef::new(id, None),
                nanovg_context: nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES),
                caption: caption,
                framebuffer_size: winsize,
                mouse_pos: (0, 0),
                mouse_state: 0,
                modifiers: 0,
                drag_active: false,
                drag_widget: None,
                last_interaction: sdl2_sys::sdl::SDL_GetTicks(),
                process_events: true,
                background: (0.3, 0.3, 0.3),
                pixel_ratio: 0.0
            };

            screen.widget.set_size(winsize);
            screen
        }
    }

    pub fn draw_widgets(&self) {
        if !self.widget.visible() {
            return
        }

        self.nanovg_context.begin_frame(self.widget.size().0, self.widget.size().1, 1.0);

        self.widget.draw(&self.nanovg_context);

        self.nanovg_context.end_frame();
    }

    pub fn set_background(&mut self, background_color: (f32, f32, f32)) {
        self.background = background_color;
    }

    pub fn nanovg_context(&self) -> &nanovg::Context {
        &self.nanovg_context
    }
}
