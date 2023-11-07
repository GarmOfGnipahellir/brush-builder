use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Vec3>,
}

impl Polygon {
    pub fn new(points: &[Vec3]) -> Self {
        Self {
            points: points.to_vec(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BrushMesh {
    pub polys: Vec<Polygon>,
}
