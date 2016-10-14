/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use na::Vector3;
use na::Matrix3;
use na::Determinant;
use objects::Object;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn is_hit(&mut self, obj: &Box<Object>) -> bool {
        for _ in {0 .. 15} {
            let dis = obj.distance(&self.origin);
            self.origin += self.dir * dis;
        }
        obj.distance(&self.origin) < 0.001
    }
}

