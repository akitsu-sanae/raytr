/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use na::Vector3;
use objects::Object;

const EPS: f32 = 0.01;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
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
}

