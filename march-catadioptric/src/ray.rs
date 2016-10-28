/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use na::{Vector3, dot, Norm};
use objects::Object;
use light::Light;


const EPS: f32 = 0.01;

#[derive(Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn color(&self, objs: &Vec<Box<Object> >, light: &Light, counter: i32) -> Option<Vector3<f32> > {
        if let Some(collision) = self.collision(objs) {
            let (pos, obj) = collision;
            let n = (*obj).normal(&pos);
            let l = pos - light.origin;
            let cos = -dot(&n, &l) / n.norm() / l.norm();
            let ld = cos;
            let ls = cos*cos*cos*cos*cos*cos*cos;
            let la = 0.1;
            let mut result = obj.color() * (ld+ls+la);
            if counter > 0 {
                let g = (obj.refractive_index()*obj.refractive_index() - cos*cos - 1.0).sqrt();
                let t = Ray {
                    origin: pos,
                    dir: (l + (cos - g)*n)/n
                };
                let r = Ray {
                    origin: pos,
                    dir: l + 2.0*cos*n,
                };
                let (t, r) = (t.color(objs, light, counter-1), (r.color(objs, light, counter-1)));
                if let Some(color) = t {
                    result += color * 0.5;
                };
                if let Some(color) = r {
                    result += color * 0.5;
                };
            }
            Some(result)
        } else {
            None
        }
    }

    fn collision<'a>(&'a self, objs: &'a Vec<Box<Object> >) -> Option<(Vector3<f32>, &Box<Object>)> {
        let mut pos = self.origin.clone();
        for _ in {0 .. 15} {
            let dis = objs.iter().map(|obj| obj.distance(&pos)).fold(0.0/0.0, |m, v| {
                v.min(m)
            });
            pos += self.dir * dis;
        }
        let res_obj = objs.iter().find(|obj| {
            obj.distance(&pos) < EPS
        });
        if let Some(obj) = res_obj {
            Some((pos, obj))
        } else {
            None
        }
    }
}

