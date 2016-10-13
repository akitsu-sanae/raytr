/*============================================================================
  Copyright (C) 2016 akitsu sanae
  https://github.com/akitsu-sanae/raytr
  Distributed under the Boost Software License, Version 1.0. (See accompanying
  file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt)
============================================================================*/


use na::Vector3;
use na::Matrix3;
use na::Determinant;
use triangle::Triangle;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub dir: Vector3<f32>,
}

impl Ray {
    pub fn is_hit(&self, triangle: &Triangle) -> bool {
        let a = triangle.points[1] - triangle.points[0];
        let b = triangle.points[2] - triangle.points[0];
        let diff = self.origin - triangle.points[0];
        let s_mat = [
            diff.as_ref(),
            b.as_ref(),
            self.dir.as_ref()] as [&[f32; 3]; 3];
        let t_mat = [
            a.as_ref(),
            diff.as_ref(),
            self.dir.as_ref()] as [&[f32; 3]; 3];

        let s = Matrix3::<f32>::new(
            s_mat[0][0], s_mat[1][0], s_mat[2][0],
            s_mat[0][1], s_mat[1][1], s_mat[2][1],
            s_mat[0][2], s_mat[1][2], s_mat[2][2]).determinant();

        let t = Matrix3::<f32>::new(
            t_mat[0][0], t_mat[1][0], t_mat[2][0],
            t_mat[0][1], t_mat[1][1], t_mat[2][1],
            t_mat[0][2], t_mat[1][2], t_mat[2][2]).determinant();
        // let t = Matrix3::<f32>::from([a.as_ref(), diff.as_ref(), self.dir.as_ref()]).determinant();
        if s < 0.0 || s > 1.0 { return false }
        if t < 0.0 || t > 1.0 { return false }
        if s+t < 0.0 || s+t > 1.0 { return false }
        true
    }
}

