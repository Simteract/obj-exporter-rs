extern crate obj_exporter as obj;

use std::path::Path;
use obj::{ObjSet, Object, Vertex, Geometry, Shape, Primitive};

pub fn main() {
  let set = ObjSet {
      material_library: None,
      objects: vec![
        Object {
          name: "Square1".to_owned(),
          vertices: vec![
            (-1, -1, 0),
            (1, -1, 0),
            (1, 1, 0),
            (-1, 1, 0)
          ].into_iter().map(|(x, y, z)| {
            Vertex {
              x: x as f64,
              y: y as f64,
              z: z as f64,
            }
          }).collect(),
          tex_vertices: vec![],
          normals: vec![],
          geometry: vec![
            Geometry {
              material_name: None,
              shapes: vec![
                (0, 1, 2),
                (0, 2, 3)
              ].into_iter().map(|(x, y, z)| {
                Shape {
                  primitive: Primitive::Triangle(
                    (x, None, None),
                    (y, None, None),
                    (z, None, None)
                  ),
                  groups: vec![],
                  smoothing_groups: vec![],
                }
              }).collect(),
            }
          ],
        },
        Object {
          name: "Square2".to_owned(),
          vertices: vec![
            (1, -1, 0),
            (2, -1, 0),
            (2, 1, 0),
            (1, 1, 0)
          ].into_iter().map(|(x, y, z)| {
            Vertex {
              x: x as f64,
              y: y as f64,
              z: z as f64,
            }
          }).collect(),
          tex_vertices: vec![],
          normals: vec![],
          geometry: vec![
            Geometry {
              material_name: None,
              shapes: vec![
                (0, 1, 2),
                (0, 2, 3)
              ].into_iter().map(|(x, y, z)| {
                Shape {
                  primitive: Primitive::Triangle(
                    (x, None, None),
                    (y, None, None),
                    (z, None, None)
                  ),
                  groups: vec![],
                  smoothing_groups: vec![],
                }
              }).collect(),
            }
          ],
        }
      ],
    };

    obj::export_to_file(&set, Path::new("output_multiple.obj")).unwrap();
}
