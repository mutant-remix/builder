use resvg::{
    render,
    FitTo,
    tiny_skia::{Pixmap, Transform},
    usvg::{Tree}
};

use image::{RgbaImage};

pub fn rasterise_svg(tree: &Tree, size: u32) -> RgbaImage {
    let mut pixmap = Pixmap::new(size, size).unwrap();
    render(
        &tree,
        FitTo::Original,
        Transform::default().pre_scale(size as f32 / 32.0, size as f32 / 32.0),
        pixmap.as_mut()
    ).unwrap();

    let data = pixmap.data();
    let rgba_image = RgbaImage::from_raw(
        size,
        size,
        data.to_vec()
    ).unwrap();

    rgba_image
}
