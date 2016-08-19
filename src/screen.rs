extern crate sdl2;
extern crate sdl2_sys;
extern crate nanovg;

use std::fmt;
use widget::{Widget, WidgetObj, WidgetObjRef};

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

impl Widget for ScreenObj {
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
        self.draw_widgets();
    }
}

impl ScreenObj {
    pub fn new(id: String, caption: String, window: &mut sdl2::video::WindowRef) -> ScreenObj {

        let winsize = window.size();
        window.set_title(&caption);

        unsafe {
            let mut screen: ScreenObj = ScreenObj {
                widget: WidgetObj::new(id),
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
