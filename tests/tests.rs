extern crate obj_exporter as obj;

use obj::{ObjSet, Object, Vertex, Geometry, Shape, Primitive};

#[test]
pub fn test_square() {
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
          tex_vertices: vec!(),
          normals: vec!(),
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
                        (x, None, None),
                        (y, None, None),
                        (z, None, None)),
                    groups: vec!(),
                    smoothing_groups: vec!(),
                  })
                .collect()
            }
          )
        }
      ]
    };

    let expected = 
r#"o Square
v -1.000000 -1.000000 0.000000
v 1.000000 -1.000000 0.000000
v 1.000000 1.000000 0.000000
v -1.000000 1.000000 0.000000
f 1 2 3
f 1 3 4
"#;

    let result = obj::export(&set);
    assert_eq!(result, expected);
}

//#[test]
pub fn test_cube() {
    let set = ObjSet {
      material_library: None,
      objects: vec![
        Object {
          name: "Cube".to_owned(),
          vertices:
            vec!((-1, -1,  1),
                 (-1, -1, -1),
                 ( 1, -1, -1),
                 ( 1, -1,  1),
                 (-1,  1,  1),
                 (-1,  1, -1),
                 ( 1,  1, -1),
                 ( 1,  1,  1))
            .into_iter()
            .map(|(x, y, z)|
              Vertex {
                x: x as f64,
                y: y as f64,
                z: z as f64
              })
            .collect(),
          tex_vertices: vec!(),
          normals: vec!(),
          geometry: vec!(
            Geometry {
              material_name: None,
              shapes:
                vec!((0, 4, 5),
                     (0, 5, 1),
                     (1, 5, 6),
                     (1, 6, 2),
                     (2, 6, 7),
                     (2, 7, 3),
                     (3, 7, 4),
                     (3, 4, 0),
                     (3, 0, 1),
                     (3, 1, 2),
                     (4, 7, 6),
                     (4, 6, 5))
                .into_iter()
                .map(|(x, y, z)|
                  Shape {
                    primitive:
                      Primitive::Triangle(
                        (x, None, None),
                        (y, None, None),
                        (z, None, None)),
                    groups: vec!(),
                    smoothing_groups: vec!(),
                  })
                .collect()
            }
          )
        }
      ]
    };

    let expected = 
r#"
o Cube
v -1.000000 -1.000000 1.000000
v -1.000000 -1.000000 -1.000000
v 1.000000 -1.000000 -1.000000
v 1.000000 -1.000000 1.000000
v -1.000000 1.000000 1.000000
v -1.000000 1.000000 -1.000000
v 1.000000 1.000000 -1.000000
v 1.000000 1.000000 1.000000
f 5 6 2 1
f 6 7 3 2
f 7 8 4 3
f 8 5 1 4
f 1 2 3 4
f 8 7 6 5
"#;
    let result = obj::export(&set);
    assert_eq!(result, expected);
}
