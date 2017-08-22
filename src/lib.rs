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
    let uvs = object.tex_vertices.iter().map(serialize_uv).join("");
    assert_eq!(object.geometry.len(), 1);
    let faces = object.geometry[0].shapes.iter().map(serialize_shape).join("");
    format!("{}{}{}{}", name, vertices, uvs, faces)
}

fn serialize_vertex(v: &Vertex) -> String {
    format!("v {:.6} {:.6} {:.6}\n", v.x, v.y, v.z)
}

fn serialize_uv(uv: &TVertex) -> String {
    if uv.w == 0.0 {
        format!("vt {:.6} {:.6}\n", uv.u, uv.v)
    } else {
        format!("vt {:.6} {:.6} {:.6}\n", uv.u, uv.v, uv.w)
    }
}

fn serialize_shape(shape: &Shape) -> String {
    match shape.primitive {
        Primitive::Point(vtn) => format!("f {}\n", serialize_vtn(vtn)),
        Primitive::Line(vtn1, vtn2) => format!("f {} {}\n", serialize_vtn(vtn1), serialize_vtn(vtn2)),
        Primitive::Triangle(vtn1, vtn2, vtn3) => format!("f {} {} {}\n", serialize_vtn(vtn1), serialize_vtn(vtn2), serialize_vtn(vtn3)),
    }
}

fn serialize_vtn(vtn: VTNIndex) -> String {
    match vtn {
        (vi, None, None) => format!("{}", vi + 1),
        (vi, Some(ti), None) => format!("{}/{}", vi + 1, ti + 1),
        (vi, Some(ti), Some(ni)) => format!("{}/{}/{}", vi + 1, ti + 1, ni + 1),
        (vi, None, Some(ni)) => format!("{}//{}", vi + 1, ni + 1),
    }
}

#[cfg(test)]
mod tests {
}
