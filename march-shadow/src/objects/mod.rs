/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/

use na::Vector3;

pub mod sphere;

pub trait Object {
    fn distance(&self, p: &Vector3<f32>) -> f32;
    fn normal(&self, p: &Vector3<f32>) -> Vector3<f32>;
}

