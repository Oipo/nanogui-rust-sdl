extern crate nanovg;
extern crate sdl2;
extern crate sdl2_sys;

use std::fmt;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use self::sdl2::keyboard::{Mod, Scancode};
use self::sdl2::mouse::Mouse;
use self::sdl2_sys::keycode::SDL_Keymod;
use common::Cursor;
use theme::Theme;
use layout::Layout;
use window::Window;
use screen::Screen;

pub struct WidgetObj {
    pub parent: Option<Weak<RefCell<Widget>>>,
    pub children: Vec<Rc<RefCell<Widget>>>,
    pub theme: Option<Rc<RefCell<Theme>>>,
    pub layout: Option<Box<Layout>>,
    pub id: String,
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub fixed_size: (u32, u32),
    pub visible: bool,
    pub enabled: bool,
    pub focused: bool,
    pub mouse_focus: bool,
    pub tooltip: String,
    pub font_size: Option<u32>,
    pub cursor: Cursor
}

pub trait Widget {
    fn parent(&self) -> Option<&Weak<RefCell<Widget>>>;
    unsafe fn set_parent(&mut self, Option<Rc<RefCell<Widget>>>);
    fn children(&self) -> Vec<Rc<RefCell<Widget>>>;
    unsafe fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<Widget>>>;

    // get/set
    fn id(&self) -> String;
    fn set_id(&mut self, String);
    fn pos(&self) -> (u32, u32);
    fn set_pos(&mut self, p: (u32, u32));
    fn size(&self) -> (u32, u32);
    fn set_size(&mut self, s: (u32, u32));
    fn fixed_size(&self) -> (u32, u32);
    fn set_fixed_size(&mut self, s: (u32, u32));
    fn font_size(&self) -> u32;
    fn set_font_size(&mut self, s: Option<u32>);
    fn theme(&self) -> Option<&Rc<RefCell<Theme>>>;
    fn set_theme(&mut self, theme: Option<Rc<RefCell<Theme>>>);
    fn enabled(&self) -> bool;
    fn set_enabled(&mut self, enabled: bool);
    fn tooltip(&self) -> String;
    fn set_tooltip(&mut self, tooltip: String);
    fn visible(&self) -> bool;
    fn set_visible(&mut self, bool);
    fn focused(&self) -> bool;
    fn set_focused(&mut self, bool);
    fn layout(&self) -> Option<&Box<Layout>>;
    fn set_layout(&mut self, Option<Box<Layout>>);
    fn cursor(&self) -> Cursor;
    fn set_cursor(&mut self, Cursor);

    // misc
    fn absolute_position(&self) -> (u32, u32);
    fn visible_recursive(&self) -> bool;
    fn contains(&self, p: (u32, u32)) -> bool;
    fn request_focus(&self);
    fn preferred_size(&self, &nanovg::Context) -> (u32, u32);
    fn perform_layout(&self, &nanovg::Context);
    fn draw(&self, nanovg_context: &nanovg::Context);

    // events
    fn mouse_button_event(&self, (u32, u32), Mouse, bool, SDL_Keymod) -> bool;
    fn mouse_motion_event(&self, (u32, u32), (u32, u32), Mouse, SDL_Keymod) -> bool;
    fn mouse_drag_event(&self, (u32, u32), (u32, u32), Mouse, SDL_Keymod) -> bool;
    fn mouse_enter_event(&mut self, (u32, u32), bool) -> bool;
    fn scroll_event(&self, (u32, u32), (u32, u32)) -> bool;
    fn focus_event(&mut self, bool) -> bool;
    fn keyboard_event(&self, Mod, Option<Scancode>, bool, SDL_Keymod) -> bool;
    fn keyboard_character_event(&self, u32) -> bool;


    // casts
    fn as_window(&self) -> Option<&Window>;
    fn as_screen(&self) -> Option<&Screen>;

    // widget_container functions:
    //fn push_child()
    //remove_child_by_child()
    //remove_child_by_id()
    //fn find_widget()
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

    // get/set

    fn id(&self) -> String {
        self.id.clone()
    }

    fn set_id(&mut self, id: String) {
        self.id = id;
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

    fn font_size(&self) -> u32 {
        match self.font_size {
            Some(val) => val,
            None => { match self.theme {
                Some(ref theme_val) => theme_val.borrow().standard_font_size(),
                None => 0u32
            } }
        }
    }

    fn set_font_size(&mut self, s: Option<u32>) {
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

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn focused(&self) -> bool {
        self.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn layout(&self) -> Option<&Box<Layout>> {
        self.layout.as_ref()
    }

    fn set_layout(&mut self, layout: Option<Box<Layout>>) {
        self.layout = layout;
    }

    fn cursor(&self) -> Cursor {
        self.cursor
    }

    fn set_cursor(&mut self, cursor: Cursor) {
        self.cursor = cursor;
    }

    // misc

    fn absolute_position(&self) -> (u32, u32) {
        if let Some(ref val) = self.parent {
            if let Some(ref val_upgraded) = val.upgrade() {
                let (par_x, par_y) = val_upgraded.borrow().absolute_position();
                return (par_x + self.pos.0, par_y + self.pos.1)
            }
        }

        return self.pos.clone();
    }

    fn preferred_size(&self, nanovg_context: &nanovg::Context) -> (u32, u32) {
        match self.layout {
            Some(ref val) => val.preferred_size(nanovg_context, self),
            None => self.size
        }
    }

    fn perform_layout(&self, nanovg_context: &nanovg::Context) {
        match self.layout {
            Some(ref val) => val.perform_layout(nanovg_context, self),
            None => {
                for child in &self.children {
                    let ps = child.borrow().preferred_size(nanovg_context);
                    let fs = child.borrow().fixed_size();
                    let mut target_size = [0u32, 0u32];

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

                    child.borrow_mut().set_size((target_size[0], target_size[1]));
                    child.borrow().perform_layout(nanovg_context);
                }
            }
        }
    }

    fn draw(&self, nanovg_context: &nanovg::Context) {
        if cfg!(feature = "draw-widget-box") {
            nanovg_context.stroke_width(1.0);
            nanovg_context.begin_path();
            nanovg_context.rect(self.pos.0 as f32 - 0.5, self.pos.1 as f32 - 0.5, self.size.0 as f32 + 1.0, self.size.1 as f32 + 1.0);
            nanovg_context.stroke_color(nanovg::Color::rgba(255, 0, 0, 255));
            nanovg_context.stroke();
        }

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
        return p.0 >= self.pos.0 && p.1 >= self.pos.1 && p.0 < self.pos.0 + self.size.0 && p.1 < self.pos.1 + self.size.1;
    }

    fn request_focus(&self) {
        let mut widget: Option<Weak<RefCell<Widget>>> = self.parent.clone();
        loop {
            if let Some(val) = widget {
                if let Some(parent) = val.upgrade() {
                    if let Some(screen) = parent.borrow().as_screen() {
                        screen.update_focus(self);
                        break;
                    } else {
                        match parent.borrow().parent() {
                            Some(val) => widget = Some(val.clone()),
                            None => break
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    // events

    fn mouse_button_event(&self, p: (u32, u32), button: Mouse, down: bool, mods: SDL_Keymod) -> bool {
        let adjusted_pos = (p.0 - self.pos.0, p.1 - self.pos.1);

        for child in &self.children {
            if child.borrow().visible() && child.borrow().contains(adjusted_pos) && child.borrow().mouse_button_event(adjusted_pos, button, down, mods) {
                return true;
            }
        }

        match button {
            Mouse::Left => {
                if down && !self.focused {
                    self.request_focus();
                }
            },
            _ => {}
        }

        return false;
    }

    fn mouse_motion_event(&self, p: (u32, u32), rel: (u32, u32), button: Mouse, mods: SDL_Keymod) -> bool {
        let adjusted_pos = (p.0 - self.pos.0, p.1 - self.pos.1);
        let prev_adjusted_pos = (p.0 - self.pos.0 - rel.0, p.1 - self.pos.1 - rel.0);

        for child in &self.children {
            if !child.borrow().visible() {
                continue
            }

            let contained: bool = child.borrow().contains(adjusted_pos);
            let prev_contained: bool = child.borrow().contains(prev_adjusted_pos);

            if contained != prev_contained {
                child.borrow_mut().mouse_enter_event(p, contained);
            }

            if (contained || prev_contained) && child.borrow().mouse_motion_event(adjusted_pos, rel, button, mods) {
                return true;
            }
        }

        return false;
    }

    fn mouse_drag_event(&self, _: (u32, u32), _: (u32, u32), _: Mouse, _: SDL_Keymod) -> bool {
        false
    }

    fn mouse_enter_event(&mut self, _: (u32, u32), enter: bool) -> bool {
        self.mouse_focus = enter;
        false
    }

    fn scroll_event(&self, p: (u32, u32), rel: (u32, u32)) -> bool {
        let adjusted_pos = (p.0 - self.pos.0, p.1 - self.pos.1);

        for child in &self.children {
            if !child.borrow().visible() {
                continue
            }

            if child.borrow().contains(adjusted_pos) && child.borrow().scroll_event(adjusted_pos, rel) {
                return true;
            }
        }

        return false;
    }

    fn focus_event(&mut self, focused: bool) -> bool {
        self.focused = focused;
        false
    }

    fn keyboard_event(&self, _: Mod, _: Option<Scancode>, _: bool, _: SDL_Keymod) -> bool {
        false
    }

    fn keyboard_character_event(&self, _: u32) -> bool {
        false
    }

    // casts

    fn as_window(&self) -> Option<&Window> {
        None
    }

    fn as_screen(&self) -> Option<&Screen> {
        None
    }
}

impl Drop for WidgetObj {
    fn drop(&mut self) {
        println!("dropping widgetobj {}", self.id);
        while let Some(child) = self.children.pop() {
            unsafe {
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
            layout: None,
            id: id,
            pos: (0, 0),
            size: (0, 0),
            fixed_size: (0, 0),
            visible: true,
            enabled: true,
            focused: false,
            mouse_focus: false,
            tooltip: String::new(),
            font_size: Some(12),
            cursor: Cursor::Arrow
        }
    }
}
