use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct BrushMesh {
    pub polys: Vec<Polygon>,
}

impl BrushMesh {
    pub fn from_planes(planes: &[Plane]) -> Self {
        Self {
            polys: find_polys(planes),
        }
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut verts = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut indices = Vec::new();

        for p in &self.polys {
            let first = verts.len() as u32;
            for &v in &p.verts {
                verts.push(v);
                normals.push(p.normal);
                uvs.push(Vec2::ZERO);
            }
            for i in 1..p.verts.len() - 1 {
                indices.push(first);
                indices.push(first + i as u32);
                indices.push(first + 1 + i as u32);
            }
        }

        Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, verts)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
            .with_indices(Some(bevy::render::mesh::Indices::U32(indices)))
    }
}

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
pub struct Polygon {
    pub verts: Vec<Vec3>,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub bitangent: Vec3,
}

impl Polygon {
    pub fn from_plane(plane: &Plane) -> Self {
        let normal = plane.normal;
        let tangent = if normal.dot(Vec3::Y).abs() > 0.5 {
            normal.cross(Vec3::X)
        } else {
            normal.cross(Vec3::Y)
        };
        let bitangent = tangent.cross(normal);
        Self {
            verts: Vec::new(),
            normal,
            tangent,
            bitangent,
        }
    }

    pub fn center(&self) -> Vec3 {
        self.verts.iter().sum::<Vec3>() / self.verts.len() as f32
    }

    pub fn sort_verts(&mut self) {
        let center = self.center();

        self.verts.sort_by(|&a, &b| {
            let l = a - center;
            let r = b - center;

            let ldt = l.dot(self.tangent);
            let ldb = l.dot(self.bitangent);

            let rdt = r.dot(self.tangent);
            let rdb = r.dot(self.bitangent);

            let la = ldb.atan2(ldt);
            let ra = rdb.atan2(rdt);

            ra.total_cmp(&la)
        })
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

pub fn is_pos_in_hull(planes: &[Plane], pos: Vec3) -> bool {
    for plane in planes {
        let proj = plane.normal.dot(pos);
        let dist = plane.distance;

        if proj > dist && proj - dist > 0.0 {
            return false;
        }
    }
    true
}

pub fn find_polys(planes: &[Plane]) -> Vec<Polygon> {
    let mut polys = planes
        .iter()
        .map(|p| Polygon::from_plane(p))
        .collect::<Vec<_>>();

    let n = planes.len();
    for i in 0..n - 2 {
        for j in i..n - 1 {
            for k in j..n {
                if i == j || i == k || j == k {
                    continue;
                }

                let (p0, p1, p2) = (&planes[i], &planes[j], &planes[k]);
                let Some(pos) = plane_intersection(p0, p1, p2) else {
                    continue;
                };

                if !is_pos_in_hull(planes, pos) {
                    continue;
                }

                polys[i].verts.push(pos);
                polys[j].verts.push(pos);
                polys[k].verts.push(pos);
            }
        }
    }

    for p in &mut polys {
        p.sort_verts();
    }

    polys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_intersection() {
        assert_eq!(
            plane_intersection(
                &Plane::new(Vec3::X, 0.0),
                &Plane::new(Vec3::Y, 0.0),
                &Plane::new(Vec3::Z, 0.0)
            ),
            Some(Vec3::ZERO)
        );
        assert_eq!(
            plane_intersection(
                &Plane::new(Vec3::X, 1.0),
                &Plane::new(Vec3::Y, 0.0),
                &Plane::new(Vec3::Z, 0.0)
            ),
            Some(Vec3::X)
        );
        assert_eq!(
            plane_intersection(
                &Plane::new(Vec3::X, 0.0),
                &Plane::new(Vec3::Y, 1.0),
                &Plane::new(Vec3::Z, 0.0)
            ),
            Some(Vec3::Y)
        );
        assert_eq!(
            plane_intersection(
                &Plane::new(Vec3::X, 0.0),
                &Plane::new(Vec3::Y, 0.0),
                &Plane::new(Vec3::Z, 1.0)
            ),
            Some(Vec3::Z)
        );
        assert_eq!(
            plane_intersection(
                &Plane::new(Vec3::X, 0.0),
                &Plane::new(Vec3::X, 0.0),
                &Plane::new(Vec3::Z, 0.0)
            ),
            None
        );
    }
}
