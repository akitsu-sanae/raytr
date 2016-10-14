/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use na::Vector3;
use objects::Object;

const EPS: f32 = 0.01;

#[derive(Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn collision_pos(&self, obj: &Box<Object>) -> Option<Vector3<f32> > {
        let mut pos = self.origin.clone();
        for _ in {0 .. 15} {
            let dis = obj.distance(&pos);
            pos += self.dir * dis;
        }
        if obj.distance(&pos) < EPS {
            Some(pos)
        } else {
            None
        }

    }
}

