extern crate wavefront_obj;
extern crate itertools;

use std::path::Path;
use std::io;
use itertools::Itertools;

pub use wavefront_obj::obj;
pub use obj::{ObjSet, Object, Shape, Geometry, TVertex, Vertex, Primitive, VertexIndex, TextureIndex, NormalIndex, VTNIndex};

pub fn export(obj_set: &ObjSet) -> String {
    obj_set.objects.iter().map(serialize_object).join("")
}

pub fn export_to_file(_obj_set: &ObjSet, _path: &Path) -> Result<(), io::Error> {
    Ok(())
}

fn serialize_object(object: &Object) -> String {
    let name = format!("o {}\n", object.name);
    let vertices = object.vertices.iter().map(serialize_vertex).join("");
    assert_eq!(object.geometry.len(), 1);
    let faces = object.geometry[0].shapes.iter().map(serialize_shape).join("");
    format!("{}{}{}", name, vertices, faces)
}

fn serialize_vertex(v: &Vertex) -> String {
    format!("v {:.6} {:.6} {:.6}\n", v.x, v.y, v.z)
}

fn serialize_shape(shape: &Shape) -> String {
    match shape.primitive {
        Primitive::Point((vi, _, _)) => format!("f {}\n", vi + 1),
        Primitive::Line((vi1, _, _), (vi2, _, _)) => format!("f {} {}\n", vi1 + 1, vi2 + 1),
        Primitive::Triangle((vi1, _, _), (vi2, _, _), (vi3, _, _)) => format!("f {} {} {}\n", vi1 + 1, vi2 + 1, vi3 + 1),
    }
}

#[cfg(test)]
mod tests {
}
