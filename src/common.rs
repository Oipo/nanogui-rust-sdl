use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color (r,g,b,a) : ({},{},{},{})", self.r, self.g, self.b, self.a)
    }
}

impl PartialEq<Color> for (f32, f32, f32, f32) {
    fn eq(&self, other: &Color) -> bool {
        self.0 == other.r && self.1 == other.g && self.2 == other.b && self.3 == other.a
    }
}

impl PartialEq<(f32, f32, f32, f32)> for Color {
    fn eq(&self, other: &(f32, f32, f32, f32)) -> bool {
        self.r == other.0 && self.g == other.1 && self.b == other.2 && self.a == other.3
    }
}

impl Color {
    pub fn new() -> Color {
        Color {
            r: 0f32,
            g: 0f32,
            b: 0f32,
            a: 0f32
        }
    }

    pub fn from_intensity(intensity: f32, alpha: f32) -> Color {
        Color {
            r: intensity,
            g: intensity,
            b: intensity,
            a: alpha,
        }
    }
}

#[macro_export]
macro_rules! impl_get_set {
    ($var_name:ident, $t:ty) => {
        interpolate_idents! {
            pub fn $var_name(&self) -> $t {
                self.$var_name
            }
            pub fn [set_ $var_name](&mut self, new_val: $t) {
                self.$var_name = new_val
            }
        }
    }
}

#[macro_export]
macro_rules! impl_get_set_borrow {
    ($var_name:ident, $t:ty) => {
        interpolate_idents! {
            pub fn $var_name(&self) -> &$t {
                &self.$var_name
            }
            pub fn [set_ $var_name](&mut self, new_val: $t) {
                self.$var_name = new_val
            }
        }
    }
}

#[macro_export]
macro_rules! impl_get_set_clone {
    ($var_name:ident, $t:ty) => {
        interpolate_idents! {
            pub fn $var_name(&self) -> $t {
                self.$var_name.clone()
            }
            pub fn [set_ $var_name](&mut self, new_val: $t) {
                self.$var_name = new_val
            }
        }
    }
}
