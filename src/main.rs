extern crate sdl2;

use sdl2::render::BlendMode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use rgsl::{integration::*, Pow};

pub fn main() -> Result<(), String> {
    let a =  3i32;
    let a2 = (2 * a) as u32;

    let mut arr: [[(f64, f64); 150]; 150] = [[(0.0, 0.0); 150]; 150];
    let mut u_max = 0.0;
    let mut u_min = 1e15;
    for x in 0..100 {
        for y in 0..100 {
            let (u, phi) = calc((x as u32 * a2) as f64 - 300.0, (y as u32 * a2) as f64 - 300.0, 1000.0, 1e6);
            if u > u_max {
                u_max = u;
            } else if u < u_min {
                u_min = u;
            }
            arr[x][y] = (u, phi);
        }
    }
    let du = u_max - u_min;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Diffraction", 600, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
    canvas.set_blend_mode(BlendMode::Blend);

    canvas.clear();
    for x in 0..100 {
        for y in 0..100 {
            let (u, _phi) = arr[x][y];
            let x = a2 as i32 * x as i32;
            let y = a2 as i32 *y as i32;
            let c = (((u - u_min) / du * 5.0)).exp();
            // let c= u *3.0;
            canvas.set_draw_color(Color::RGBA(255, 255, 255, c as u8));
            canvas.fill_rect(Rect::new(x + a, y + a, a2, a2)).unwrap();
        }
    }
    canvas.present();

    std::thread::sleep(std::time::Duration::from_secs(1000));

    Ok(())
}

fn u0(x: f64, y: f64) -> f64 {
    let cond = x.pow_2() + y.pow_2() < 9e-4;
    if cond {
        1e-4
    } else {
        0.0
    }
}
const A: f64 = 1000.0;

fn calc(x: f64, y: f64, z: f64, k: f64) -> (f64, f64) {
    let (u_re, err_re, _abs, _asc) = qk61(
        |x0: f64| {
            let (res, _err, _abs, _asc) = qk61(
                |y0: f64| {
                    let r2 = (x - x0).pow_2() + (y - y0).pow_2() + z.pow_2();
                    let r = r2.sqrt();
                    let kr = k*r;
                    u0(x0, y0) * z / r2 * (kr.cos() - kr.sin() / kr)
                },
                A,
                -A,
            );
            res
        },
        A,
        -A,
    );

    let (u_im, err_im, _abs, _asc) = qk61(
        |x0: f64| {
            let (res, _err, _abs, _asc) = qk61(
                |y0: f64| {
                    let r2 = (x - x0).pow_2() + (y - y0).pow_2() + z.pow_2();
                    let r = r2.sqrt();
                    let kr = k*r;
                    u0(x0, y0) * z / r2 * (kr.sin() - kr.cos() / kr)
                },
                A,
                -A,
            );
            res
        },
        A,
        -A,
    );

    let u = u_re.hypot(u_im) * k / std::f64::consts::TAU;
    // let relerr = ((u_re * err_re).pow_2() + (u_im * err_im).pow_2()).sqrt() / u.pow_2();
    // dbg!(u, relerr);
    dbg!(u);
    // let phi = u_im.atan2(u_re);
    // (u, phi)
    (u, 0.0)
}
