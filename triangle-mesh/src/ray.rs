
use na::{Vector3, Matrix3, Determinant};
use mesh::Mesh;

#[derive(Clone)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn collision(&self, mesh: &Mesh) -> Option<Vector3<f32>> {
        for face in mesh.faces.iter() {
            let node_poss: Vec<_> = face.node_ids
                .iter()
                .map(|&i| mesh.nodes[(i - 1) as usize].position)
                .collect();
            if collision_impl(self, node_poss) {
                return Some(Vector3::new(200.0, 100.0, 0.0));
            }
        }
        None
    }
}

fn collision_impl(ray: &Ray, nodes: Vec<Vector3<f32>>) -> bool {
    let a = nodes[1] - nodes[0];
    let b = nodes[2] - nodes[0];
    let r = -ray.direction;
    let denominator = Matrix3::<f32>::from(&[
       a.as_ref().clone(),
       b.as_ref().clone(),
       r.as_ref().clone()
    ]).determinant();
    if denominator <= 0.0 {
        return false
    }

    let d = ray.origin - nodes[0];
    let u = Matrix3::<f32>::from(&[
       d.as_ref().clone(),
       b.as_ref().clone(),
       r.as_ref().clone()
    ]).determinant() / denominator;
    let v = Matrix3::<f32>::from(&[
       a.as_ref().clone(),
       d.as_ref().clone(),
       r.as_ref().clone()
    ]).determinant() / denominator;
    let t = Matrix3::<f32>::from(&[
       a.as_ref().clone(),
       b.as_ref().clone(),
       d.as_ref().clone()
    ]).determinant();

    if u < 0.0 || u > 1.0 { return false }
    if v < 0.0 || v > 1.0 { return false }
    if u+v < 0.0 || u+v > 1.0 { return false }
    if t < 0.0 { return false }
    true
}
 
