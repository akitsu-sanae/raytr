
use na::Vector3;

#[derive(Debug)]
pub struct Node {
    pub position: Vector3<f32>,
}

impl Node {
    pub fn from_vec(p: Vec<f32>) -> Self {
        Node { position: Vector3::new(p[0], p[1], p[2]) }
    }
}
