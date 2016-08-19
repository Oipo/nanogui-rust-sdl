trait Color {
    fn contrasting_color() -> (f32, f32, f32, f32);
}

impl Color for (f32, f32, f32, f32) {
    fn contrasting_color() -> (f32, f32, f32, f32) {
        (0.0, 0.0, 0.0, 0.0)
    }
}
