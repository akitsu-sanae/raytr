/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use na::{Vector3, dot, Norm};
use objects::Object;

pub struct Board {
    normal: Vector3<f32>,
    diff: f32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            normal: Vector3::new(0.0, 1.0, 0.0),
            diff: -1.0,
        }
    }
}

impl Object for Board {
    fn distance(&self, p: &Vector3<f32>) -> f32 {
        (dot(&self.normal, &p) + self.diff).abs() / self.normal.norm()
    }

    fn normal(&self, _: &Vector3<f32>) -> Vector3<f32> {
        -self.normal
    }

    fn refractive_index(&self) -> f32 {
        1.4
    }

    fn color(&self) -> Vector3<f32> {
        Vector3::new(120.0, 64.0, 64.0)
    }
}

