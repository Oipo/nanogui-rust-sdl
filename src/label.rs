extern crate nanovg;
extern crate sdl2;
extern crate sdl2_sys;

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use self::sdl2::keyboard::{Mod, Scancode};
use self::sdl2::mouse::Mouse;
use self::sdl2_sys::keycode::SDL_Keymod;
use common::Cursor;
use widget::{Widget, WidgetObj};
use theme::Theme;
use layout::Layout;
use window::Window;
use screen::Screen;

pub struct Label {
    widget: WidgetObj,
    caption: String,
    font: String,
    nanovg_font: Option<nanovg::Font>,
    color: (u8, u8, u8, u8)
}

impl Widget for Label {
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

    // get/set

    fn id(&self) -> String {
        self.widget.id.clone()
    }

    fn set_id(&mut self, id: String) {
        self.widget.id = id;
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

    fn font_size(&self) -> u32 {
        self.widget.font_size()
    }

    fn set_font_size(&mut self, s: Option<u32>) {
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

    fn set_visible(&mut self, visible: bool) {
        self.widget.visible = visible;
    }

    fn focused(&self) -> bool {
        self.widget.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.widget.focused = focused;
    }

    fn layout(&self) -> Option<&Box<Layout>> {
        self.widget.layout.as_ref()
    }

    fn set_layout(&mut self, layout: Option<Box<Layout>>) {
        self.widget.layout = layout;
    }

    fn cursor(&self) -> Cursor {
        self.widget.cursor
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        self.widget.cursor = cursor;
    }

    // misc

    fn perform_layout(&self, nanovg_context: &nanovg::Context) {
        self.widget.perform_layout(nanovg_context);
    }

    fn preferred_size(&self, nanovg_context: &nanovg::Context) -> (u32, u32) {
        if self.caption.len() == 0 {
            return (0, 0);
        }

        nanovg_context.font_face(&self.font);
        nanovg_context.font_size(self.font_size() as f32);

        if self.widget.fixed_size.0 > 0 {
            let mut bounds = [0f32; 4];
            nanovg_context.text_align(nanovg::LEFT | nanovg::MIDDLE);
            nanovg_context.text_box_bounds(self.widget.pos.0 as f32, self.widget.pos.1 as f32, self.widget.fixed_size.0 as f32, &self.caption, &mut bounds);
            return (self.widget.fixed_size.0, (bounds[3] - bounds[1]) as u32)
        } else {
            nanovg_context.text_align(nanovg::LEFT | nanovg::MIDDLE);
            let size_x = nanovg_context.text_bounds(0u32 as f32, 0u32 as f32, &self.caption, None);
            let theme = self.widget.theme.as_ref().unwrap();
            return (size_x as u32, theme.borrow().standard_font_size())
        }
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        self.widget.draw(nanovg_context);

        /*println!("drawing label: \"{}\" - \"{}\" - {} - ({} {} {} {}) - ({} {}) - ({} {}) - @{} {}",
            self.font, self.caption,
            self.widget.font_size(),
            self.color.0 as f32, self.color.1, self.color.2, self.color.3,
            self.widget.size.0, self.widget.size.1,
            self.widget.fixed_size.0 as f32, self.widget.fixed_size.1,
            self.widget.pos.0, self.widget.pos.1);*/

        match self.nanovg_font {
            Some(ref val) => nanovg_context.font_face_id(val),
            None => nanovg_context.font_face(&self.font)
        }

        nanovg_context.font_size(self.widget.font_size() as f32);
        nanovg_context.fill_color(nanovg::Color::rgba(self.color.0, self.color.1, self.color.2, self.color.3));
        if self.widget.fixed_size.0 > 0 {
            nanovg_context.text_align(nanovg::LEFT | nanovg::TOP);
            nanovg_context.text_box(self.widget.pos.0 as f32, self.widget.pos.1 as f32, self.widget.fixed_size.0 as f32, &self.caption);
        } else {
            nanovg_context.text_align(nanovg::LEFT | nanovg::TOP);
            nanovg_context.text(self.widget.pos.0 as f32, (self.widget.pos.1 + self.widget.size.1 / 2) as f32, &self.caption);
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
        return p.0 >= self.widget.pos.0 && p.1 >= self.widget.pos.1 && p.0 < self.widget.pos.0 + self.widget.size.0 && p.1 < self.widget.pos.1 + self.widget.size.1;
    }

    fn request_focus(&self) {
        self.widget.request_focus();
    }

    // events

    fn mouse_button_event(&self, p: (u32, u32), button: Mouse, down: bool, mods: SDL_Keymod) -> bool {
        self.widget.mouse_button_event(p, button, down, mods)
    }

    fn mouse_motion_event(&self, p: (u32, u32), rel: (u32, u32), button: Mouse, mods: SDL_Keymod) -> bool {
        self.widget.mouse_motion_event(p, rel, button, mods)
    }

    fn mouse_drag_event(&self, p: (u32, u32), rel: (u32, u32), button: Mouse, mods: SDL_Keymod) -> bool {
        self.widget.mouse_drag_event(p, rel, button, mods)
    }

    fn mouse_enter_event(&mut self, p: (u32, u32), enter: bool) -> bool {
        self.widget.mouse_enter_event(p, enter)
    }

    fn scroll_event(&self, p: (u32, u32), rel: (u32, u32)) -> bool {
        self.widget.scroll_event(p, rel)
    }

    fn focus_event(&mut self, focused: bool) -> bool {
        self.widget.focus_event(focused)
    }

    fn keyboard_event(&self, key: Mod, scancode: Option<Scancode>, pressed: bool, mods: SDL_Keymod) -> bool {
        self.widget.keyboard_event(key, scancode, pressed, mods)
    }

    fn keyboard_character_event(&self, codepoint: u32) -> bool {
        self.widget.keyboard_character_event(codepoint)
    }

    // casts

    fn as_window(&self) -> Option<&Window> {
        None
    }

    fn as_screen(&self) -> Option<&Screen> {
        None
    }
}

impl Label {
    pub fn new_create_font(id: String, caption: String, font_filename: String, nanovg_context: &nanovg::Context) -> Rc<RefCell<Label>> {
        Rc::new(RefCell::new(Label {
            widget: WidgetObj::new(id),
            caption: caption,
            color: (255, 255, 255, 125),
            font: font_filename.clone(),
            nanovg_font: Some(nanovg_context.create_font(&font_filename, &font_filename).unwrap())
        }))
    }

    pub fn new(id: String, caption: String, font_filename: String, font: Option<nanovg::Font>) -> Rc<RefCell<Label>> {
        Rc::new(RefCell::new(Label {
            widget: WidgetObj::new(id),
            caption: caption,
            color: (255, 255, 255, 125),
            font: font_filename.clone(),
            nanovg_font: font
        }))
    }

    impl_get_set!(color, (u8, u8, u8, u8));
}
