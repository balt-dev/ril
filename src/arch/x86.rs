#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;


#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse")]
#[target_feature(enable = "fma")]
pub unsafe fn _merge_impl(original: crate::pixel::Rgba, other: crate::pixel::Rgba) -> crate::pixel::Rgba {
        // Optimize for common cases
        if other.a == 255 {
            return other;
        } else if other.a == 0 {
            return original;
        }

        let mut base = [0_f32; 4];

        _mm_store_ps(
            base.as_mut_ptr(),
            _mm_div_ps(_mm_setr_ps(original.r as f32, original.g as f32, original.b as f32, original.a as f32), _mm_set1_ps(255.)),
        );

        let [base_r, base_g, base_b, base_a] = base;

        let mut overlay = [0_f32; 4];

        _mm_store_ps(
            overlay.as_mut_ptr(),
            _mm_div_ps(_mm_setr_ps(other.r as f32, other.g as f32, other.b as f32, other.a as f32), _mm_set1_ps(255.)),
        );

        let [overlay_r, overlay_g, overlay_b, overlay_a] = overlay;

        let a_diff = 1. - overlay_a;

        let mut overlay_rgba = [0_f32; 4];

        _mm_store_ps(
            overlay_rgba.as_mut_ptr(),
            _mm_mul_ps(
                _mm_setr_ps(overlay_r, overlay_g, overlay_b, base_a),
                _mm_setr_ps(overlay_a, overlay_a, overlay_a, a_diff),
            ),
        );

        let [overlay_r, overlay_g, overlay_b, a_ratio] = overlay_rgba;

        let mut rgba = [0_f32; 4];

        _mm_store_ps(
            rgba.as_mut_ptr(),
            _mm_fmadd_ps(
                _mm_setr_ps(a_ratio, a_ratio, a_ratio, a_diff),
                _mm_setr_ps(base_r, base_g, base_b, base_a),
                _mm_setr_ps(overlay_r, overlay_g, overlay_b, overlay_a),
            ),
        );

        let [r, g, b, a] = rgba;

        let mut res = [0_f32; 4];

        _mm_store_ps(
            res.as_mut_ptr(),
            _mm_mul_ps(
                _mm_div_ps(_mm_setr_ps(r, g, b, a), _mm_setr_ps(a, a, a, 1.)),
                _mm_set1_ps(255.),
            ),
        );

        let [r, g, b, a] = res;

        crate::pixel::Rgba { r: r as u8, g: g as u8, b: b as u8, a: a as u8 }
}
