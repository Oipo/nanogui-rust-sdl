extern crate nanoguirustsdl;

use nanoguirustsdl::common::Color;

#[test]
fn color_creation_test() {
    let color = Color::from_intensity(5f32, 6f32);
    assert_eq!(color, (5f32, 5f32, 5f32, 6f32));
}
