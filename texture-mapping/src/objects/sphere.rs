/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use std::path::Path;
use std::f32::consts::PI;

use image::{DynamicImage, GenericImage, Pixel, open};
use na::Vector3;
use objects::Object;

const EPS: f32 = 0.01;

use std::sync::Mutex;
lazy_static! {
    static ref TEXTURE: Mutex<DynamicImage> =
        Mutex::new(open(&Path::new("sphere.png")).unwrap());
}

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32
}

impl Object for Sphere {
    fn distance(&self, p: &Vector3<f32>) -> f32 {
        let d = self.center - p;
        let len2 = d.x*d.x + d.y*d.y + d.z*d.z;
        len2.sqrt() - self.radius
    }

    fn normal(&self, p: &Vector3<f32>) -> Vector3<f32> {
        let dvx = Vector3::new(EPS, 0.0, 0.0);
        let dvy = Vector3::new(0.0, EPS, 0.0);
        let dvz = Vector3::new(0.0, 0.0, EPS);
        let dx = self.distance(&(p + dvx)) - self.distance(&(p - dvx));
        let dy = self.distance(&(p + dvy)) - self.distance(&(p - dvy));
        let dz = self.distance(&(p + dvz)) - self.distance(&(p - dvz));
        Vector3::new(dx, dy, dz)
    }

    fn refractive_index(&self) -> f32 {
        1.4
    }

    fn color(&self, pos: &Vector3<f32>) -> Vector3<f32> {
        let pos = self.center - pos;
        let pos = pos / self.radius;
        // x = sin(theta) cos(phi)
        // y = sin(theta) sin(phi)
        // z = cos(theta)
        let phi = (pos.z / pos.x).atan();
        let theta = pos.y.asin();

        let mut u  = (phi + PI) / (2.0 * PI) + 0.8;
        let mut v = (theta + PI / 2.0) / PI + 0.8;
        if u >= 1.0 { u -= 1.0; }
        if v >= 1.0 { v -= 1.0; }

        let x = (u * TEXTURE.lock().unwrap().width() as f32) as u32;
        let y = (v * TEXTURE.lock().unwrap().height() as f32) as u32;
        let color = TEXTURE.lock().unwrap().get_pixel(x, y).to_rgb();
        Vector3::new(color[0] as f32, color[1] as f32, color[2] as f32)
    }
}

