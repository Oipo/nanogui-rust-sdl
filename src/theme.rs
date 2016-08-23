extern crate nanovg;

use common::{Color};
use resources;

pub struct Theme {
    font_normal: nanovg::Font,
    font_bold: nanovg::Font,
    font_icons: nanovg::Font,

    standard_font_size: i32,
    button_font_size: i32,
    textbox_font_size: i32,
    window_corner_radius: i32,
    window_header_height: i32,
    window_dropshadow_size: i32,
    button_corner_radius: i32,

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
            font_normal: nanovg_context.create_font_mem("sans", resources::SANS_FONT).unwrap(),
            font_bold: nanovg_context.create_font_mem("sans-bold", resources::SANS_BOLD_FONT).unwrap(),
            font_icons: nanovg_context.create_font_mem("icons", resources::SANS_ICONS_FONT).unwrap(),

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

    impl_get_set_borrow!(font_normal, nanovg::Font);
    impl_get_set_borrow!(font_bold, nanovg::Font);
    impl_get_set_borrow!(font_icons, nanovg::Font);

    impl_get_set!(standard_font_size, i32);
    impl_get_set!(button_font_size, i32);
    impl_get_set!(textbox_font_size, i32);
    impl_get_set!(window_corner_radius, i32);
    impl_get_set!(window_header_height, i32);
    impl_get_set!(window_dropshadow_size, i32);
    impl_get_set!(button_corner_radius, i32);

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
