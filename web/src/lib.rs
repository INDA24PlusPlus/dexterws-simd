use mandelbrot::{mandelbrot_simd, mandelbrot_sisd, Span};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData, Performance};

#[wasm_bindgen]
pub fn draw_mandelbrot(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    simd: bool,
    max_iteration: u32,
    min_r: f32,
    max_r: f32,
    min_i: f32,
    max_i: f32,
    r: u32,
    g: u32,
    b: u32,
) -> JsValue {
    let window = web_sys::window().unwrap();
    let performance = window.performance().unwrap();
    performance.mark("start").unwrap();
    let span = Span::new(min_r, max_r, min_i, max_i);
    let colors = [r, g, b];
    let mut buffer = if simd {
        mandelbrot_simd((width, height), max_iteration, span, colors)
    } else {
        mandelbrot_sisd((width, height), max_iteration, span, colors)
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
