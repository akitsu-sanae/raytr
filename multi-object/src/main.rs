/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(box_syntax)]

extern crate num_traits;
extern crate image;
extern crate nalgebra as na;

use std::fs::File;
use std::path::Path;
use num_traits::Float;

use na::Vector3;

mod objects;
mod ray;
mod light;
use objects::sphere::Sphere;
use objects::Object;
use ray::Ray;
use light::Light;

fn main() {
    let size = (800, 800);
    let scale = (4.0 / size.0 as f32, 4.0 / size.1 as f32);
    let mut buf = image::ImageBuffer::new(size.0, size.1);
    let sphere1: Box<Object> = box Sphere {
        center: Vector3::new(0.5, 0.5, 0.5),
        radius: 2.0,
    };
    let sphere2: Box<Object> = box Sphere {
        center: Vector3::new(0.0, 0.5, 0.0),
        radius: 1.5,
    };
    let objs = vec![sphere1, sphere2];
    let light = Light {
        origin: Vector3::new(0.0, 0.0, -2.0),
    };
    let ray_origin = Vector3::new(0.5, 0.5, -2.0);
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let ray = Ray {
            origin: ray_origin,
            dir: Vector3::new(
                x as f32 * scale.0 - 2.0,
                y as f32 * scale.1 - 2.0,
                1.0),
        };
        if let Some(color) = ray.color(&objs, &light) {
            *pixel = image::Rgb {
                data: [
                    255.0.min(0.0.max(color)) as u8,
                    255.0.min(0.0.max(color)) as u8,
                    255.0.min(0.0.max(color)) as u8]
            };
        }
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    let _ = image::ImageRgb8(buf).save(fout, image::PNG);
}

