extern crate wavefront_obj;
extern crate itertools;

use std::path::Path;
use std::io;
use std::io::Write;
use std::fs;
use itertools::Itertools;

pub use wavefront_obj::obj;
pub use obj::{ObjSet, Object, Shape, Geometry, TVertex, Vertex, Primitive, VertexIndex, TextureIndex, NormalIndex, VTNIndex};

pub fn export(obj_set: &ObjSet) -> String {
    obj_set.objects.iter().map(serialize_object).join("")
}

pub fn export_to_file(obj_set: &ObjSet, path: &Path) -> Result<(), io::Error> {
    let mut file = fs::File::create(path)?;
    let content = export(obj_set);
    file.write(content.as_bytes())?;
    Ok(())
}

fn serialize_object(object: &Object) -> String {
    let name = format!("o {}\n", object.name);
    let vertices = object.vertices.iter().map(|v| serialize_vertex(v, "v")).join("");
    let uvs = object.tex_vertices.iter().map(serialize_uv).join("");
    let normals = object.normals.iter().map(|n| serialize_vertex(n, "vn")).join("");
    assert_eq!(object.geometry.len(), 1);
    let faces = object.geometry[0].shapes.iter().map(serialize_shape).join("");
    format!("{}{}{}{}{}", name, vertices, uvs, normals, faces)
}

fn serialize_vertex(v: &Vertex, prefix: &str) -> String {
    format!("{} {:.6} {:.6} {:.6}\n", prefix, v.x, v.y, v.z)
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
        Primitive::Point(vtn) => format!("p {}\n", serialize_vtn(vtn)),
        Primitive::Line(vtn1, vtn2) => format!("l {} {}\n", serialize_vtn(vtn1), serialize_vtn(vtn2)),
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
