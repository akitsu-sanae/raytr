
#[derive(Debug)]
pub struct Face {
    pub node_ids: [u32; 3],
}

impl Face {
    pub fn from_vec(node_ids: Vec<u32>) -> Self {
        Face { node_ids: [node_ids[0], node_ids[1], node_ids[2]] }
    }
}
