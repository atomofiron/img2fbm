use image::DynamicImage;
use image::imageops::FilterType;

pub trait Resizing {
    fn resize(&self, nwidth: u32, nheight: u32, fill: bool, filter: FilterType) -> Self;
}

impl Resizing for DynamicImage {

    fn resize(&self, to_width: u32, to_height: u32, fill: bool, filter: FilterType) -> Self {
        let (width, height) = resize_dimensions(self.width(), self.height(), to_width, to_height, fill);
        return self.resize_exact(width, height, filter);
    }
}

// image::math::utils::resize_dimensions()
fn resize_dimensions(
    width: u32,
    height: u32,
    nwidth: u32,
    nheight: u32,
    fill: bool,
) -> (u32, u32) {
    let wratio = nwidth as f64 / width as f64;
    let hratio = nheight as f64 / height as f64;

    let ratio = if fill {
        f64::max(wratio, hratio)
    } else {
        f64::min(wratio, hratio)
    };

    let nw = 1.max((width as f64 * ratio).round() as u64);
    let nh = 1.max((height as f64 * ratio).round() as u64);

    if nw > u64::from(u32::MAX) {
        let ratio = u32::MAX as f64 / width as f64;
        (u32::MAX, 1.max((height as f64 * ratio).round() as u32))
    } else if nh > u64::from(u32::MAX) {
        let ratio = u32::MAX as f64 / height as f64;
        (1.max((width as f64 * ratio).round() as u32), u32::MAX)
    } else {
        (nw as u32, nh as u32)
    }
}
