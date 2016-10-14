/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

extern crate image;
extern crate nalgebra as na;

use std::fs::File;
use std::path::Path;

use na::Vector3;

mod triangle;
use triangle::Triangle;
mod ray;
use ray::Ray;

fn main() {
    // let max_iterations = 256u16;
    let size = (800, 800);
    let scale = (4.0 / size.0 as f32, 4.0 / size.1 as f32);
    let mut buf = image::ImageBuffer::new(size.0, size.1);
    let triangle = Triangle {
        points: [
            Vector3::new(0.5, 0.0, 0.5),
            Vector3::new(0.0, 1.0, 0.5),
            Vector3::new(1.0, 1.0, 0.5)]
    };
    let ray_origin = Vector3::new(0.5, 0.5, -1.0);
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let ray = Ray {
            origin: ray_origin,
            dir: Vector3::new(
                x as f32 * scale.0 - 2.0,
                y as f32 * scale.1 - 2.0,
                1.0),
        };
        if ray.is_hit(&triangle) {
            *pixel = image::Luma([255 as u8]);
        }
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    let _ = image::ImageLuma8(buf).save(fout, image::PNG);
}

