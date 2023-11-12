use num_complex::Complex64;
use rayon::prelude::*;
extern crate image;

fn main() {
    let height: u32 = 10000;
    let width: u32 = 10000;
    let start_x: f64 = -1.78;
    let start_y: f64 = 0.01905;
    let box_w: f64 = 0.037;
    let box_h: f64 = box_w;
    let iters: u32 = 1000;
    //
    // *full set*
    //let height: u32 = 5000;
    //let width: u32 = 5000;
    //let start_x: f64 = -2.0;
    //let start_y: f64 = 2.0;
    //let box_w: f64 = 4.0;
    //let box_h: f64 = 4.0;
    //let iters: u32 = 1000;
    let set = count_set(width,height,start_x,start_y,box_w,box_h,iters);
    draw_counts(set,width,height);
    //print_set(set,width,height);
}

#[allow(dead_code)]
fn count_set(w: u32, h: u32, start_x: f64, start_y: f64, box_width: f64, box_height: f64, count: u32) -> Vec<Vec<i32>> {
    let mut set: Vec<Vec<i32>> = Vec::new();
    println!("w = {}, h = {}, w*h = {}",w,h,w*h);
    for i in 0..h {
        set.push(vec![i as i32]);
    }
    let diff_x: f64 = box_width / w as f64;
    let diff_y: f64 = box_height / h as f64;
    set.par_iter_mut().for_each(|row| {
        let y = *row.get(0).unwrap();
        row.pop();
        for i in 0..w {
            let comp = Complex64::new(start_x + diff_x * i as f64, start_y - diff_y * y as f64);
            row.push(get_count(comp,count));
        }
    });
    return set;
}

fn get_count(c: Complex64, iter_count: u32) -> i32 {
    let mut i = 1;
    let mut z = c;
    loop {
        if (z.re * z.re + z.im*z.im) >= 4.0 {
            return i
        }
        i = i + 1;
        if i == (iter_count as i32) {
            return -1
        }
        z = z*z + c;
    }
}

#[allow(dead_code)]
fn draw_counts(set: Vec<Vec<i32>>, w: u32, h: u32) {
    let mut imgbuf = image::ImageBuffer::new(w, h);
    for (x,y,pix) in imgbuf.enumerate_pixels_mut() {
        let row = set.get(y as usize).unwrap();
        let pnt = row.get(x as usize).unwrap();
        if *pnt == -1 {
            *pix = image::Rgb([0 as u8,0 as u8,0 as u8]);
        } else {
            let b: u8 = (((pnt * 4) + 64) % 256) as u8;
            let r: u8 = (((pnt * 2) + 64) % 256) as u8;
            *pix = image::Rgb([b,0,r]);
        }
    }

    imgbuf.save("img.png").unwrap();
}

#[allow(dead_code)]
fn print_set(set: Vec<i32>,w: u32, h: u32) {
    for i in 0..h {
        for j in 0..w {
            if *set.get((i * w + j) as usize).unwrap() == -1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}
