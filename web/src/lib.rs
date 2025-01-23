use mandelbrot::{mandelbrot_simd, mandelbrot_sisd};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData, Performance};

#[wasm_bindgen]
pub fn draw_mandelbrot(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    simd: bool,
    max_iteration: u32,
) -> JsValue {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();
    performance.mark("start").unwrap();
    let mut buffer = if simd {
        mandelbrot_simd((width, height), max_iteration)
    } else {
        mandelbrot_sisd((width, height), max_iteration)
    };
    performance.mark("end").unwrap();
    let _ = performance.measure_with_start_mark_and_end_mark("mandelbrot", "start", "end");
    let time_taken = performance
        .get_entries_by_name("mandelbrot")
        .iter()
        .next()
        .unwrap();
    performance.clear_marks();
    performance.clear_measures();

    let data =
        ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut buffer), width, height).unwrap();
    ctx.put_image_data(&data, 0.0, 0.0).unwrap();
    time_taken
}
