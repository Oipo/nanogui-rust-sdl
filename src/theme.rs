extern crate nanovg;

use common::{Color};
use resources;

pub struct Theme {
    font_normal: Option<nanovg::Font>,
    font_bold: Option<nanovg::Font>,
    font_icons: Option<nanovg::Font>,

    standard_font_size: u32,
    button_font_size: u32,
    textbox_font_size: u32,
    window_corner_radius: u32,
    window_header_height: u32,
    window_dropshadow_size: u32,
    button_corner_radius: u32,

    dropshadow: Color,
    transparent: Color,
    border_dark: Color,
    border_light: Color,
    border_medium: Color,
    text_color: Color,
    disabled_text_color: Color,
    text_color_shadow: Color,
    icon_color: Color,

    button_gradient_top_focused: Color,
    button_gradient_bot_focused: Color,
    button_gradient_top_unfocused: Color,
    button_gradient_bot_unfocused: Color,
    button_gradient_top_pushed: Color,
    button_gradient_bot_pushed: Color,

    window_fill_unfocused: Color,
    window_fill_focused: Color,
    window_title_unfocused: Color,
    window_title_focused: Color,

    window_header_gradient_top: Color,
    window_header_gradient_bot: Color,
    window_header_sep_top: Color,
    window_header_sep_bot: Color,

    window_popup: Color,
    window_popup_transparent: Color,
}

impl Theme {
    pub fn new(nanovg_context: &nanovg::Context) -> Theme {

        Theme {
            font_normal: Some(nanovg_context.create_font_mem("sans", resources::SANS_FONT).unwrap()),
            font_bold: Some(nanovg_context.create_font_mem("sans-bold", resources::SANS_BOLD_FONT).unwrap()),
            font_icons: Some(nanovg_context.create_font_mem("icons", resources::SANS_ICONS_FONT).unwrap()),

            standard_font_size: 16,
            button_font_size: 20,
            textbox_font_size: 20,
            window_corner_radius: 2,
            window_header_height: 30,
            window_dropshadow_size: 10,
            button_corner_radius: 2,

            dropshadow: Color::from_intensity(0f32, 128f32),
            transparent: Color::from_intensity(0f32, 0f32),
            border_dark: Color::from_intensity(29f32, 255f32),
            border_light: Color::from_intensity(92f32, 255f32),
            border_medium: Color::from_intensity(35f32, 255f32),
            text_color: Color::from_intensity(255f32, 160f32),
            disabled_text_color: Color::from_intensity(255f32, 80f32),
            text_color_shadow: Color::from_intensity(0f32, 160f32),
            icon_color: Color::from_intensity(255f32, 160f32),

            button_gradient_top_focused: Color::from_intensity(64f32, 255f32),
            button_gradient_bot_focused: Color::from_intensity(48f32, 255f32),
            button_gradient_top_unfocused: Color::from_intensity(74f32, 255f32),
            button_gradient_bot_unfocused: Color::from_intensity(58f32, 255f32),
            button_gradient_top_pushed: Color::from_intensity(41f32, 255f32),
            button_gradient_bot_pushed: Color::from_intensity(29f32, 255f32),

            window_fill_unfocused: Color::from_intensity(43f32, 230f32),
            window_fill_focused: Color::from_intensity(45f32, 230f32),
            window_title_unfocused: Color::from_intensity(220f32, 160f32),
            window_title_focused: Color::from_intensity(255f32, 190f32),

            window_header_gradient_top: Color::from_intensity(74f32, 255f32),
            window_header_gradient_bot: Color::from_intensity(58f32, 255f32),
            window_header_sep_top: Color::from_intensity(92f32, 255f32),
            window_header_sep_bot: Color::from_intensity(29f32, 255f32),

            window_popup: Color::from_intensity(50f32, 255f32),
            window_popup_transparent: Color::from_intensity(50f32, 0f32),
        }
    }

    pub fn new_debug() -> Theme {

        Theme {
            font_normal: None,
            font_bold: None,
            font_icons: None,

            standard_font_size: 16,
            button_font_size: 20,
            textbox_font_size: 20,
            window_corner_radius: 2,
            window_header_height: 30,
            window_dropshadow_size: 10,
            button_corner_radius: 2,

            dropshadow: Color::from_intensity(0f32, 128f32),
            transparent: Color::from_intensity(0f32, 0f32),
            border_dark: Color::from_intensity(29f32, 255f32),
            border_light: Color::from_intensity(92f32, 255f32),
            border_medium: Color::from_intensity(35f32, 255f32),
            text_color: Color::from_intensity(255f32, 160f32),
            disabled_text_color: Color::from_intensity(255f32, 80f32),
            text_color_shadow: Color::from_intensity(0f32, 160f32),
            icon_color: Color::from_intensity(255f32, 160f32),

            button_gradient_top_focused: Color::from_intensity(64f32, 255f32),
            button_gradient_bot_focused: Color::from_intensity(48f32, 255f32),
            button_gradient_top_unfocused: Color::from_intensity(74f32, 255f32),
            button_gradient_bot_unfocused: Color::from_intensity(58f32, 255f32),
            button_gradient_top_pushed: Color::from_intensity(41f32, 255f32),
            button_gradient_bot_pushed: Color::from_intensity(29f32, 255f32),

            window_fill_unfocused: Color::from_intensity(43f32, 230f32),
            window_fill_focused: Color::from_intensity(45f32, 230f32),
            window_title_unfocused: Color::from_intensity(220f32, 160f32),
            window_title_focused: Color::from_intensity(255f32, 190f32),

            window_header_gradient_top: Color::from_intensity(74f32, 255f32),
            window_header_gradient_bot: Color::from_intensity(58f32, 255f32),
            window_header_sep_top: Color::from_intensity(92f32, 255f32),
            window_header_sep_bot: Color::from_intensity(29f32, 255f32),

            window_popup: Color::from_intensity(50f32, 255f32),
            window_popup_transparent: Color::from_intensity(50f32, 0f32),
        }
    }

    pub fn font_normal(&self) -> &nanovg::Font {
        match self.font_normal {
            Some(ref val) => val,
            None => panic!("debug mode")
        }
    }

    pub fn set_font_normal(&mut self, new_val: nanovg::Font) {
        self.font_normal = Some(new_val);
    }

    pub fn font_bold(&self) -> &nanovg::Font {
        match self.font_bold {
            Some(ref val) => val,
            None => panic!("debug mode")
        }
    }

    pub fn set_font_bold(&mut self, new_val: nanovg::Font) {
        self.font_bold = Some(new_val);
    }

    pub fn font_icons(&self) -> &nanovg::Font {
        match self.font_icons {
            Some(ref val) => val,
            None => panic!("debug mode")
        }
    }

    pub fn set_font_icons(&mut self, new_val: nanovg::Font) {
        self.font_icons = Some(new_val);
    }

    impl_get_set!(standard_font_size, u32);
    impl_get_set!(button_font_size, u32);
    impl_get_set!(textbox_font_size, u32);
    impl_get_set!(window_corner_radius, u32);
    impl_get_set!(window_header_height, u32);
    impl_get_set!(window_dropshadow_size, u32);
    impl_get_set!(button_corner_radius, u32);

    impl_get_set!(dropshadow, Color);
    impl_get_set!(transparent, Color);
    impl_get_set!(border_dark, Color);
    impl_get_set!(border_light, Color);
    impl_get_set!(border_medium, Color);
    impl_get_set!(text_color, Color);
    impl_get_set!(disabled_text_color, Color);
    impl_get_set!(text_color_shadow, Color);
    impl_get_set!(icon_color, Color);

    impl_get_set!(button_gradient_top_focused, Color);
    impl_get_set!(button_gradient_bot_focused, Color);
    impl_get_set!(button_gradient_top_unfocused, Color);
    impl_get_set!(button_gradient_bot_unfocused, Color);
    impl_get_set!(button_gradient_top_pushed, Color);
    impl_get_set!(button_gradient_bot_pushed, Color);

    impl_get_set!(window_fill_unfocused, Color);
    impl_get_set!(window_fill_focused, Color);
    impl_get_set!(window_title_unfocused, Color);
    impl_get_set!(window_title_focused, Color);

    impl_get_set!(window_header_gradient_top, Color);
    impl_get_set!(window_header_gradient_bot, Color);
    impl_get_set!(window_header_sep_top, Color);
    impl_get_set!(window_header_sep_bot, Color);

    impl_get_set!(window_popup, Color);
    impl_get_set!(window_popup_transparent, Color);
}
