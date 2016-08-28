#![feature(box_syntax)]

extern crate gl;
extern crate sdl2;
extern crate nanoguirustsdl;
extern crate nanovg;

use std::process;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{GLProfile, WindowRef, GLContext};
use nanoguirustsdl::screen::{ScreenObj};
use nanoguirustsdl::label::{LabelObj};
use nanoguirustsdl::widget::{Widget};

fn init_gl(window: &WindowRef) -> GLContext {
    unsafe {gl::FrontFace(gl::CCW)};
    unsafe {gl::Enable(gl::DEPTH_TEST)};
    unsafe {gl::Enable(gl::SCISSOR_TEST)};
    unsafe {gl::DepthFunc(gl::LEQUAL)};
    unsafe {gl::FrontFace(gl::CCW)};
    unsafe {gl::Enable(gl::CULL_FACE)};
    unsafe {gl::CullFace(gl::BACK)};

    let gl_context = window.gl_create_context().unwrap();
    match window.gl_make_current(&gl_context) {
        Err(val) => {
            println!("make_current error: {}", val);
            process::exit(1);
        },
        _ => {}
    }

    gl_context
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_flags().debug().forward_compatible().set();
    gl_attr.set_context_version(3, 2);
    gl_attr.set_stencil_size(8);

    let mut window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    let gl_context = init_gl(&window);

    let mut event_pump = sdl_context.event_pump().unwrap();

    let vg: nanovg::Context = nanovg::Context::create_gl3(nanovg::ANTIALIAS | nanovg::STENCIL_STROKES);
    let mut screen: ScreenObj = ScreenObj::new("test screen".to_string(), "Test screen".to_string(), &mut window);
    let mut label: Rc<RefCell<LabelObj>> = Rc::new(RefCell::new(LabelObj::new_create_font("test label".to_string(), "This is a label".to_string(), "Roboto-Regular.ttf".to_string(), screen.nanovg_context())));
    label.borrow_mut().set_size((200, 20));
    label.borrow_mut().set_fixed_size((200, 20));
    label.borrow_mut().set_font_size(22);
    screen.push_child(label.clone());

    let mut posx = 0;
    let mut posy = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        unsafe {gl::ClearColor(0.0, 0.0, 0.0, 0.0)};
        unsafe {gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT)};
        unsafe {gl::Enable(gl::BLEND)};
        unsafe {gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)};
        unsafe {gl::Enable(gl::CULL_FACE)};
        unsafe {gl::Disable(gl::DEPTH_TEST)};

        screen.draw(&vg);

        window.gl_swap_window();

        label.borrow_mut().set_pos((posx, posy));
        posx += 1;
        posy += 1;

        if posy > 580
        {
            posx = 0;
            posy = 0;
        }
    }
}
