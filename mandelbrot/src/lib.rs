#![feature(portable_simd)]
#![feature(test)]
use std::io::Write;
use std::simd::cmp::SimdPartialOrd;
use std::simd::{self, f32x4, u32x4};

type FloatSIMD = f32x4;
type UIntSIMD = u32x4;

pub struct Rgb {
    pub data: [u8; 3],
}

fn iter_to_color(iteration: u32, max_iteration: u32, dr: u32, dg: u32, db: u32) -> Rgb {
    if iteration == max_iteration {
        return Rgb { data: [0, 0, 0] };
    }
    let r = if dr == 0 { 0 } else { (iteration * 3) % dr };
    let g = if dg == 0 { 0 } else { ((iteration + 1) * 3) % dg };
    let b = if db == 0 { 0 } else { ((iteration + 2) * 3) % db }; 
    Rgb { data: [r as u8, g as u8, b as u8] }
}

pub struct Span {
    min_r: f32,
    max_r: f32,
    min_i: f32,
    max_i: f32,
}

impl Span {
    pub fn new(min_r: f32, max_r: f32, min_i: f32, max_i: f32) -> Self {
        Self {
            min_r,
            max_r,
            min_i,
            max_i,
        }
    }
}

pub fn mandelbrot_simd(dim: (u32, u32), max_iteration: u32, span: Span, colors: [u32; 3]) -> Vec<u8> {
    let (width, height) = dim;
    let mut buffer = Vec::with_capacity((width * height) as usize * 4);
    let mut slice = vec![0.0; 4];
    for i in 0..4 {
        slice[i] = i as f32;
    }
    let add = FloatSIMD::from_slice(&slice);
    for y in 0..height {
        for x in (0..width).step_by(4) {
            let mut x0 = FloatSIMD::splat(x as f32);
            x0 = x0 + add;
            x0 = x0 / FloatSIMD::splat(width as f32);
            x0 = x0 * FloatSIMD::splat(span.max_r - span.min_r) + FloatSIMD::splat(span.min_r);
            let y0 = FloatSIMD::splat(y as f32) / FloatSIMD::splat(height as f32)
                * FloatSIMD::splat(span.max_i - span.min_i)
                + FloatSIMD::splat(span.min_i);
            let mut x = FloatSIMD::splat(0.0);
            let mut y = FloatSIMD::splat(0.0);

            let mut iteration = UIntSIMD::splat(0);

            let threshhold = FloatSIMD::splat(4.0);

            for _ in 0..max_iteration {
                let xx = x * x;
                let yy = y * y;

                // Iteration logic from here: https://pythonspeed.com/articles/optimizing-with-simd/
                let mask = (xx + yy).simd_lt(threshhold);
                if !mask.any() {
                    break;
                }

                iteration += mask.select(UIntSIMD::splat(1), UIntSIMD::splat(0));

                // Copy paste stop

                let xy = x * y;
                x = xx - yy + x0;
                y = xy + xy + y0;
            }

            let res = iteration.as_array();
            for re in res {
                if *re == max_iteration {
                    buffer.push(0);
                    buffer.push(0);
                    buffer.push(0);
                    buffer.push(255);
                    continue;
                }
                let color = iter_to_color(*re, max_iteration, colors[0], colors[1], colors[2]);
                buffer.push(color.data[0]);
                buffer.push(color.data[1]);
                buffer.push(color.data[2]);
                buffer.push(255);
            }
        }
    }
    buffer
}

pub fn mandelbrot_sisd(dim: (u32, u32), max_iteration: u32, span: Span, colors: [u32; 3]) -> Vec<u8> {
    let (width, height) = dim;
    let mut buffer = Vec::with_capacity((width * height) as usize * 4);
    for y in 0..height {
        for x in 0..width {
            let x0 = (x as f32 / width as f32) * (span.max_r - span.min_r) + span.min_r;
            let y0 = (y as f32 / height as f32) * (span.max_i - span.min_i) + span.min_i;
            let mut x = 0.0;
            let mut y = 0.0;
            let mut iteration = 0;
            loop {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iteration += 1;
                let dist = x * x + y * y;
                if iteration == max_iteration || dist > 4.0 {
                    break;
                }
            }
            if iteration == max_iteration {
                buffer.push(0);
                buffer.push(0);
                buffer.push(0);
                buffer.push(255);
                continue;
            }
                let color = iter_to_color(iteration, max_iteration, colors[0], colors[1], colors[2]);
            buffer.push(color.data[0]);
            buffer.push(color.data[1]);
            buffer.push(color.data[2]);
            buffer.push(255);
        }
    }
    buffer
}

mod benchmarks {
    extern crate test;
    use super::*;
    use test::Bencher;

    const SPAN: Span = Span {
        min_r: -2.0,
        max_r: 1.0,
        min_i: -1.0,
        max_i: 1.0,
    };

    #[bench]
    fn bench_mandelbrot_simd(b: &mut Bencher) {
        b.iter(|| {
            mandelbrot_simd((800, 600), 500, SPAN, [255, 255, 255]);
        });
    }

    #[bench]
    fn bench_mandelbrot_sisd(b: &mut Bencher) {
        b.iter(|| {
            mandelbrot_sisd((800, 600), 500, SPAN, [255, 255, 255]);
        });
    }
}
