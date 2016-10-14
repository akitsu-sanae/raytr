/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

#![feature(box_syntax)]

extern crate image;
extern crate nalgebra as na;

use std::fs::File;
use std::path::Path;

use na::{Vector3, dot};

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
    let obj: Box<Object> = box Sphere {
        center: Vector3::new(0.5, 0.5, 0.5),
        radius: 5.0,
    };
    let light = Light {
        origin: Vector3::new(1.0, 1.0, 1.0),
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
        if let Some(collision_pos) = ray.collision_pos(&obj) {
            let n = obj.normal(&collision_pos);
            let l = light.origin - collision_pos;
            let i = dot(&n, &l);
            *pixel = image::Rgb {
                data: [
                    (i * 255.0) as u8,
                    (i * 255.0) as u8,
                    (i * 255.0) as u8]
            };
        }
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    let _ = image::ImageRgb8(buf).save(fout, image::PNG);
}

