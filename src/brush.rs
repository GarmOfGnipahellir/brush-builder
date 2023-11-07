use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

impl Plane {
    pub fn new(normal: Vec3, distance: f32) -> Self {
        Self { normal, distance }
    }
}
#[derive(Debug, Clone)]
pub struct Brush {
    pub planes: Vec<Plane>,
}

impl Brush {
    pub fn new(planes: &[Plane]) -> Self {
        Self {
            planes: planes.to_vec(),
        }
    }

    pub fn is_point_in_hull(&self, point: Vec3) -> bool {
        for plane in &self.planes {
            let proj = plane.normal.dot(point);
            let dist = plane.distance;

            if proj > dist && proj - dist > 0.0 {
                return false;
            }
        }
        true
    }

    pub fn calc_points_edges(&self) -> (Vec<Vec3>, Vec<(usize, usize)>) {
        let mut points = Vec::new();

        let n = self.planes.len();
        for i in 0..n - 2 {
            for j in i..n - 1 {
                for k in j..n {
                    if i == j || i == k || j == k {
                        continue;
                    }

                    let (p0, p1, p2) = (&self.planes[i], &self.planes[j], &self.planes[k]);
                    let Some(point) = plane_intersection(p0, p1, p2) else {
                        continue;
                    };

                    if !self.is_point_in_hull(point) {
                        continue;
                    }

                    points.push((point, [i, j, k]));
                }
            }
        }

        let mut edges = Vec::new();

        let n = points.len();
        for i in 0..n - 1 {
            for j in i..n {
                let mut num_shared = 0;

                for ip in points[i].1 {
                    for jp in points[j].1 {
                        if ip == jp {
                            num_shared += 1;
                        }
                    }
                }

                if num_shared == 2 {
                    edges.push((i, j));
                }
            }
        }

        (points.iter().map(|(p, _)| *p).collect(), edges)
    }
}

pub fn plane_intersection(plane0: &Plane, plane1: &Plane, plane2: &Plane) -> Option<Vec3> {
    let (n0, n1, n2) = (plane0.normal, plane1.normal, plane2.normal);

    let denom = n0.cross(n1).dot(n2);

    if denom.abs() <= 0.0 {
        return None;
    }

    let (d0, d1, d2) = (plane0.distance, plane1.distance, plane2.distance);

    Some((d0 * n1.cross(n2) + d1 * n2.cross(n0) + d2 * n0.cross(n1)) / denom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brush_calc_points_edges() {
        let brush = Brush::new(&[
            Plane::new(Vec3::X, 0.5),
            Plane::new(Vec3::Y, 0.5),
            Plane::new(Vec3::Z, 0.5),
            Plane::new(Vec3::NEG_X, 0.5),
            Plane::new(Vec3::NEG_Y, 0.5),
            Plane::new(Vec3::NEG_Z, 0.5),
        ]);
        let (points, edges) = brush.calc_points_edges();
        assert_eq!(points.len(), 8);
        assert_eq!(edges.len(), 12);
    }
}
