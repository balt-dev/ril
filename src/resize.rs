//! An interfacing layer between fast_image_resize and this crate.

use crate::{
    encodings::{ColorType, PixelData},
    Pixel,
};

use fast_image_resize::{
    FilterType as ResizeFilterType, Image as ResizeImage, PixelType as ResizePixelType, ResizeAlg,
    Resizer,
};
use std::num::NonZeroU32;

/// A filtering algorithm that is used to resize an image.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum FilterType {
    /// A simple nearest neighbor algorithm. Although the fastest, this gives the lowest quality
    /// resizings.
    ///
    /// When upscaling this is good if you want a "pixelated" effect with no aliasing.
    #[default]
    Nearest,
    /// A box filter algorithm. Equivalent to the [`Nearest`] filter if you are upscaling.
    Box,
    /// A bilinear filter. Calculates output pixel value using linear interpolation on all pixels.
    Bilinear,
    /// While having similar performance as the [`Bilinear`] filter, this produces a sharper and
    /// usually considered better quality image than the [`Bilinear`] filter, but **only** when
    /// downscaling. This may give worse results than bilinear when upscaling.
    Hamming,
    /// A Catmull-Rom bicubic filter, which is the most common bicubic filtering algorithm. Just
    /// like all cubic filters, it uses cubic interpolation on all pixels to calculate output
    /// pixels.
    Bicubic,
    /// A Mitchell-Netravali bicubic filter. Just like all cubic filters, it uses cubic
    /// interpolation on all pixels to calculate output pixels.
    Mitchell,
    /// A Lanczos filter with a window of 3. Calculates output pixel value using a high-quality
    /// Lanczos filter on all pixels.
    Lanczos3,
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::Bicubic
    }
}

impl From<FilterType> for ResizeAlg {
    fn from(f: FilterType) -> Self {
        type F = ResizeFilterType;

        ResizeAlg::Convolution(match f {
            FilterType::Nearest => return ResizeAlg::Nearest,
            FilterType::Box => F::Box,
            FilterType::Bilinear => F::Bilinear,
            FilterType::Hamming => F::Hamming,
            FilterType::Bicubic => F::CatmullRom,
            FilterType::Mitchell => F::Mitchell,
            FilterType::Lanczos3 => F::Lanczos3,
        })
    }
}

pub(crate) fn resize<P: Pixel>(
    data: &Vec<P>,
    src_width: NonZeroU32,
    src_height: NonZeroU32,
    dst_width: NonZeroU32,
    dst_height: NonZeroU32,
    filter: FilterType,
) -> Vec<P> {
    let (color_type, bit_depth) = data[0].as_pixel_data().type_data();
    let pixel_type = match bit_depth {
        1 | 2 | 4 | 8 => match color_type {
            ColorType::L => ResizePixelType::U8,
            ColorType::LA => ResizePixelType::U8x2,
            ColorType::Rgb => ResizePixelType::U8x3,
            ColorType::Rgba => ResizePixelType::U8x4,
            ColorType::Palette => ResizePixelType::U8,
        },
        16 => match color_type {
            ColorType::L => ResizePixelType::U16,
            ColorType::LA => ResizePixelType::U16x2,
            ColorType::Rgb => ResizePixelType::U16x3,
            ColorType::Rgba => ResizePixelType::U16x4,
            ColorType::Palette => ResizePixelType::U16,
        },
        _ => panic!("Unsupported bit depth"),
    };

    let buffer = data
        .iter()
        .flat_map(|p| p.as_pixel_data().data())
        .collect::<Vec<_>>();
    // We are able to unwrap here since we validated the buffer throughout the creation of the image.
    let image = ResizeImage::from_vec_u8(src_width, src_height, buffer, pixel_type).unwrap();
    let view = image.view();

    let mut dest = ResizeImage::new(dst_width, dst_height, pixel_type);
    let mut dst_view = dest.view_mut();

    let mut resizer = Resizer::new(filter.into());
    // The pixel type is the same, we can unwrap here
    resizer.resize(&view, &mut dst_view).unwrap();

    let bpp = color_type.channels() * ((bit_depth as usize + 7) >> 3);
    dest.into_vec()
        .chunks_exact(bpp)
        .map(|c| PixelData::from_raw(color_type, bit_depth, c).and_then(P::from_pixel_data))
        .collect::<crate::Result<Vec<_>>>()
        .unwrap()
}