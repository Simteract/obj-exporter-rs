extern crate itertools;
extern crate wavefront_obj;

use std::path::Path;
use std::io::{BufWriter, Result, Write};
use std::fs;

use wavefront_obj::obj;
pub use obj::{Geometry, GroupName, NormalIndex, ObjSet, Object, Primitive, Shape, TVertex,
              TextureIndex, VTNIndex, Vertex, VertexIndex};

/// Exports `ObjSet` to given output.
pub fn export<W: Write>(obj_set: &ObjSet, output: &mut W) -> Result<()> {
    Exporter::new(output).export(obj_set)
}

/// Exports `ObjSet`to file.
pub fn export_to_file<P: AsRef<Path>>(obj_set: &ObjSet, path: P) -> Result<()> {
    let file = fs::File::create(path)?;
    let mut buffered = BufWriter::new(file);
    export(obj_set, &mut buffered)
}

struct Exporter<'a, W: 'a + Write> {
    output: &'a mut W,
    v_base_id: usize,
    uv_base_id: usize,
    n_base_id: usize,
}

impl<'a, W: 'a + Write> Exporter<'a, W> {
    fn new(output: &'a mut W) -> Exporter<W> {
        Exporter {
            output,
            v_base_id: 1,
            uv_base_id: 1,
            n_base_id: 1,
        }
    }

    fn export(&mut self, obj_set: &ObjSet) -> Result<()> {
        for object in &obj_set.objects {
            self.serialize_object(object)?;
        }
        Ok(())
    }

    fn serialize_object(&mut self, object: &Object) -> Result<()> {
        write!(self.output, "o {}\n", object.name)?;
        self.serialize_vertex_data(object)?;
        for g in &object.geometry {
            self.serialize_geometry(g)?;
        }
        self.update_base_indices(object);
        Ok(())
    }

    fn serialize_vertex_data(&mut self, object: &Object) -> Result<()> {
        for v in &object.vertices {
            self.serialize_vertex(v, "v")?;
        }
        for uv in &object.tex_vertices {
            self.serialize_uv(uv)?;
        }
        for n in &object.normals {
            self.serialize_vertex(n, "vn")?
        }
        Ok(())
    }

    fn serialize_geometry(&mut self, geometry: &Geometry) -> Result<()> {
        for s in &geometry.shapes {
            self.serialize_shape(s)?;
        }
        Ok(())
    }

    fn serialize_vertex(&mut self, v: &Vertex, prefix: &str) -> Result<()> {
        write!(self.output, "{} {:.6} {:.6} {:.6}\n", prefix, v.x, v.y, v.z)
    }

    fn serialize_uv(&mut self, uv: &TVertex) -> Result<()> {
        if uv.w == 0.0 {
            write!(self.output, "vt {:.6} {:.6}\n", uv.u, uv.v)
        } else {
            write!(self.output, "vt {:.6} {:.6} {:.6}\n", uv.u, uv.v, uv.w)
        }
    }

    fn serialize_shape(&mut self, shape: &Shape) -> Result<()> {
        match shape.primitive {
            Primitive::Point(vtn) => {
                write!(self.output, "p")?;
                self.serialize_vtn(vtn)?;
            }
            Primitive::Line(vtn1, vtn2) => {
                write!(self.output, "l")?;
                self.serialize_vtn(vtn1)?;
                self.serialize_vtn(vtn2)?;
            }
            Primitive::Triangle(vtn1, vtn2, vtn3) => {
                write!(self.output, "f")?;
                self.serialize_vtn(vtn1)?;
                self.serialize_vtn(vtn2)?;
                self.serialize_vtn(vtn3)?;
            }
        }
        writeln!(self.output, "")
    }

    fn serialize_vtn(&mut self, vtn: VTNIndex) -> Result<()> {
        match vtn {
            (vi, None, None) => write!(self.output, " {}", vi + self.v_base_id),
            (vi, Some(ti), None) => write!(
                self.output,
                " {}/{}",
                vi + self.v_base_id,
                ti + self.uv_base_id
            ),
            (vi, Some(ti), Some(ni)) => write!(
                self.output,
                " {}/{}/{}",
                vi + self.v_base_id,
                ti + self.uv_base_id,
                ni + self.n_base_id
            ),
            (vi, None, Some(ni)) => write!(
                self.output,
                " {}//{}",
                vi + self.v_base_id,
                ni + self.n_base_id
            ),
        }
    }

    fn update_base_indices(&mut self, object: &Object) {
        self.v_base_id += object.vertices.len();
        self.uv_base_id += object.tex_vertices.len();
        self.n_base_id += object.normals.len();
    }
}
