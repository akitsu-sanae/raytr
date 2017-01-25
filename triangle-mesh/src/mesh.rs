
use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::str::FromStr;

use face::Face;
use node::Node;
use bounding_box::BoundingBox;

#[derive(Debug)]
pub struct Mesh {
    pub faces: Vec<Face>,
    pub nodes: Vec<Node>,
}

impl Mesh {
    pub fn load(filename: &str) -> Result<Mesh, Error> {
        let mut result = Mesh {
            faces: vec![],
            nodes: vec![],
        };

        let f = try!(File::open(filename));
        let file = BufReader::new(&f);
        for line in file.lines() {
            let line = line.unwrap();
            let line = line.trim();
            match line.chars().nth(0) {
                Some('#') => continue,
                Some('v') => {
                    let pos = line[1..]
                        .trim()
                        .split(' ')
                        .map(|str| f32::from_str(str).unwrap())
                        .collect();
                    result.nodes.push(Node::from_vec(pos))
                }
                Some('f') => {
                    let node_ids = line[1..]
                        .trim()
                        .split(' ')
                        .map(|str| u32::from_str(str).unwrap())
                        .collect();
                    result.faces.push(Face::from_vec(node_ids))
                }
                None => (), // skip
                _ => panic!("invalid line in loaded file: {}", line),
            }
        }
        Ok(result)
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let left = self.nodes.iter().max_by(|n1, n2| {
            n1.position[0].partial_cmp(&n2.position[0]).unwrap()
        }).unwrap().position[0];
        let right = self.nodes.iter().min_by(|n1, n2| {
            n1.position[0].partial_cmp(&n2.position[0]).unwrap()
        }).unwrap().position[0];

        let top = self.nodes.iter().min_by(|n1, n2| {
            n1.position[1].partial_cmp(&n2.position[1]).unwrap()
        }).unwrap().position[1];
        let bottom = self.nodes.iter().max_by(|n1, n2| {
            n1.position[1].partial_cmp(&n2.position[1]).unwrap()
        }).unwrap().position[1];

        let front = self.nodes.iter().min_by(|n1, n2| {
            n1.position[2].partial_cmp(&n2.position[2]).unwrap()
        }).unwrap().position[2];
        let behind = self.nodes.iter().max_by(|n1, n2| {
            n1.position[2].partial_cmp(&n2.position[2]).unwrap()
        }).unwrap().position[2];

        BoundingBox {
            top: top,
            bottom: bottom,
            left: left,
            right: right,
            front: front,
            behind: behind
        }
    }
}
