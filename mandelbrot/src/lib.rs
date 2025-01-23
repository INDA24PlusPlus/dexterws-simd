#![feature(portable_simd)]
#![feature(test)]
use std::simd::cmp::SimdPartialOrd;
use std::{io::Write};
use std::simd::{self, u32x4, f32x4};

type floatvec = f32x4;
type uintvec = u32x4;


pub struct Rgb {
    pub data: [u8; 3],
}

fn iter_to_color(iteration: u32, max_iteration: u32) -> Rgb {
    if iteration == max_iteration {
        return Rgb { data: [0, 0, 0] };
    }
    let r = ((iteration * 16) % 16) as u8;
    let g = ((iteration * 5) % 256) as u8;
    let b = (iteration % 256) as u8;
    Rgb { data: [r, g, b] }
}

pub fn mandelbrot_simd(dim: (u32, u32), max_iteration: u32) -> Vec<u8> {
    let (width, height) = dim;
    let mut buffer = Vec::with_capacity((width * height) as usize * 4);
    let mut slice = vec![0.0; 4];
    for i in 0..4 {
        slice[i] = i as f32;
    }
    let add = floatvec::from_slice(&slice);
    for y in 0..height {
        for x in (0..width).step_by(4) {
            let mut x0 = floatvec::splat(x as f32);
            x0 = x0 + add;
            x0 = x0 / floatvec::splat(width as f32);
            x0 = x0 * floatvec::splat(2.5) - floatvec::splat(2.0);
            let y0 = floatvec::splat(y as f32) / floatvec::splat(height as f32) * floatvec::splat(2.0) - floatvec::splat(1.0);
            let mut x = floatvec::splat(0.0);
            let mut y = floatvec::splat(0.0);

            let mut iteration = uintvec::splat(0);

            let threshhold = floatvec::splat(4.0);

            for _ in 0..max_iteration {
                let xx = x * x;
                let yy = y * y;

                let mask = (xx + yy).simd_lt(threshhold);
                if !mask.any() {
                    break;
                }

                iteration += mask.select(
                    uintvec::splat(1),
                    uintvec::splat(0),
                );

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
                let color = iter_to_color(*re, max_iteration);
                buffer.push(color.data[0]);
                buffer.push(color.data[1]);
                buffer.push(color.data[2]);
                buffer.push(255);
            }
        }
    }
    buffer
}

pub fn mandelbrot_sisd(dim: (u32, u32), max_iteration: u32) -> Vec<u8> {
    let (width, height) = dim;
    let mut buffer = Vec::with_capacity((width * height) as usize * 4);
    for y in 0..height {
        for x in 0..width {
            let x0 = (x as f32 / width as f32) * 2.5 - 2.0;
            let y0 = (y as f32 / height as f32) * 2.0 - 1.0;
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
            let color = iter_to_color(iteration, max_iteration);
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

    #[bench]
    fn bench_mandelbrot_simd(b: &mut Bencher) {
        b.iter(|| {
            mandelbrot_simd((800, 600), 500);
        });
    }

    #[bench]
    fn bench_mandelbrot_sisd(b: &mut Bencher) {
        b.iter(|| {
            mandelbrot_sisd((800, 600), 500);
        });
    }
}