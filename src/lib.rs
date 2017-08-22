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
    let mut exporter = Exporter::new();
    exporter.export(obj_set)
}

pub fn export_to_file(obj_set: &ObjSet, path: &Path) -> Result<(), io::Error> {
    let mut file = fs::File::create(path)?;
    let content = export(obj_set);
    file.write(content.as_bytes())?;
    Ok(())
}

struct Exporter {
    pub v_base_id: usize,
    pub uv_base_id: usize,
    pub n_base_id: usize,
}

impl Exporter {
    fn new() -> Exporter {
        Exporter {
            v_base_id: 1,
            uv_base_id: 1,
            n_base_id: 1,
        }
    }

    fn export(&mut self, obj_set: &ObjSet) -> String {
        obj_set.objects.iter().map(|o| self.serialize_object(o)).join("")
    }

    fn serialize_object(&mut self, object: &Object) -> String {
        let name = format!("o {}\n", object.name);
        let vertices = object.vertices.iter().map(|v| Exporter::serialize_vertex(v, "v")).join("");
        let uvs = object.tex_vertices.iter().map(Exporter::serialize_uv).join("");
        let normals = object.normals.iter().map(|n| Exporter::serialize_vertex(n, "vn")).join("");
        assert_eq!(object.geometry.len(), 1);
        let faces = object.geometry[0].shapes.iter().map(|s| self.serialize_shape(s)).join("");
        self.update_base_indices(object);
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

    fn serialize_shape(&self, shape: &Shape) -> String {
        match shape.primitive {
            Primitive::Point(vtn) => format!("p {}\n", self.serialize_vtn(vtn)),
            Primitive::Line(vtn1, vtn2) => format!("l {} {}\n", self.serialize_vtn(vtn1), self.serialize_vtn(vtn2)),
            Primitive::Triangle(vtn1, vtn2, vtn3) => format!("f {} {} {}\n", self.serialize_vtn(vtn1), self.serialize_vtn(vtn2), self.serialize_vtn(vtn3)),
        }
    }

    fn serialize_vtn(&self, vtn: VTNIndex) -> String {
        match vtn {
            (vi, None, None) => format!("{}", vi + self.v_base_id),
            (vi, Some(ti), None) => format!("{}/{}", vi + self.v_base_id, ti + self.uv_base_id),
            (vi, Some(ti), Some(ni)) => format!("{}/{}/{}", vi + self.v_base_id, ti + self.uv_base_id, ni + self.n_base_id),
            (vi, None, Some(ni)) => format!("{}//{}", vi + self.v_base_id, ni + self.n_base_id),
        }
    }

    fn update_base_indices(&mut self, object: &Object) {
        self.v_base_id += object.vertices.len();
        self.uv_base_id += object.tex_vertices.len();
        self.n_base_id += object.normals.len();
    }
}