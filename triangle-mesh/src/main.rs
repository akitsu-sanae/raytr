
#![feature(iter_max_by)]
#![feature(iter_min_by)]

extern crate num_traits;
extern crate image;
extern crate nalgebra as na;

use std::fs::File;
use std::path::Path;
use num_traits::float::Float;
use na::Vector3;

mod node;
mod face;
mod mesh;
mod ray;
mod bounding_box;

use ray::Ray;

fn main() {
    let mesh = mesh::Mesh::load("bunny.obj").unwrap();
    let bounding_box = mesh.bounding_box();
    println!("{:?}", bounding_box);
    println!("n-faces: {:?}", mesh.faces.len());
    println!("n-nodes: {:?}", mesh.nodes.len());

    
    let size = (800, 800);
    let mut buf = image::ImageBuffer::new(size.0, size.1);

    let scale = (4.0 / size.0 as f32, 4.0 / size.1 as f32);
    let ray_origin = Vector3::new(0.0, 0.0, -0.1);
    for (x, y, pixel) in buf.enumerate_pixels_mut() {
        let ray = Ray {
            origin: ray_origin,
            direction: Vector3::new(x as f32 * scale.0 - 2.0, y as f32 * scale.1 - 2.0, 1.0),
        };
        if let Some(color) = ray.collision(&mesh) {
            *pixel = image::Rgb {
                data: [255.0.min(0.0.max(color.x)) as u8,
                       255.0.min(0.0.max(color.y)) as u8,
                       255.0.min(0.0.max(color.z)) as u8],
            };
        }
    }

    let ref mut fout = File::create(&Path::new("output.png")).unwrap();
    image::ImageRgb8(buf).save(fout, image::PNG).unwrap();
}
