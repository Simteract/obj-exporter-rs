extern crate obj_exporter as obj;

use std::path::Path;
use obj::{ObjSet, Object, Vertex, TVertex, Geometry, Shape, Primitive};

pub fn main() {
    let set = ObjSet {
        material_library: None,
        objects: vec![
        Object {
          name: "Square".to_owned(),
          vertices:
            vec![(-1, -1,  0),
                 (1, -1, 0),
                 (1, 1, 0),
                 (-1, 1,  0)]
            .into_iter()
            .map(|(x, y, z)|
              Vertex {
                x: x as f64,
                y: y as f64,
                z: z as f64
              })
            .collect(),
          tex_vertices: vec![
            (0, 0),
            (1, 0),
            (1, 1),
            (0, 1),
          ].into_iter().map(|(u, v)|
            TVertex {
              u: u as f64,
              v: v as f64,
              w: 0.0,
            }
          ).collect(),
          normals: vec![
            Vertex {
              x: 0.0,
              y: 0.0,
              z: -1.0,
            }
          ],
          geometry: vec!(
            Geometry {
              material_name: None,
              shapes:
                vec![(0, 1, 2),
                     (0, 2, 3)]
                .into_iter()
                .map(|(x, y, z)|
                  Shape {
                    primitive:
                      Primitive::Triangle(
                        (x, Some(x), Some(0)),
                        (y, Some(y), Some(0)),
                        (z, Some(z), Some(0))),
                    groups: vec!(),
                    smoothing_groups: vec!(),
                  })
                .collect()
            }
          )
        }
      ],
    };

    obj::export_to_file(&set, Path::new("output.obj")).unwrap();
}
