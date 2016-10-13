/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use na::Vector3;
use na::dot;
use sphere::Sphere;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn is_hit(&self, sphere: &Sphere) -> bool {
        let a = dot(&self.dir, &self.dir);
        let b = dot(&self.origin, &self.dir);
        let c = dot(&self.origin, &self.origin) - sphere.radius*sphere.radius;
        let d = b*b - a*c;
        if d > 0.0 {
            -b - d.sqrt() > 0.0
        } else {
            false
        }
    }
}

