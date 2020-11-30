use std::io;

use glium::vertex::VertexBufferAny;
use glium::Display;
use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

implement_vertex!(Vertex, position, normal, texture);

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_stl<R>(data: &mut R) -> io::Result<Vec<Vertex>>
where
    R: io::Read + io::Seek
{
    let mesh = stl_io::read_stl(data)?;

    Ok(mesh.faces.iter()
        .flat_map(|triangle|
            triangle.vertices.iter()
            .map(|&vertex_index| mesh.vertices[vertex_index])
            .map(move |position| Vertex {
                position: [position[0], position[1], position[2]], // TODO https://github.com/hmeyer/stl_io/pull/9
                normal: [triangle.normal[0], triangle.normal[1], triangle.normal[2]],
                texture: [0.0, 0.0],
            })
        )
        .collect())
}


/// Returns a vertex buffer that should be rendered as `TrianglesList`.
#[allow(dead_code)]
pub fn load_wavefront<R>(display: &Display, data: &mut R) -> io::Result<VertexBufferAny> 
where
    R: io::Read
{
    // TODO catch error
    let data = obj::ObjData::load_buf(data).unwrap();

    let mut vertex_data = Vec::new();

    for object in &data.objects {
        for polygon in object.groups.iter().flat_map(|g| g.polys.iter()) {
            match polygon {
                obj::SimplePolygon(indices) => {
                    for v in indices.iter() {
                        let position = data.position[v.0];
                        let texture = v.1.map(|index| data.texture[index]);
                        let normal = v.2.map(|index| data.normal[index]);

                        let texture = texture.unwrap_or([0.0, 0.0]);
                        let normal = normal.unwrap_or([0.0, 0.0, 0.0]);

                        vertex_data.push(Vertex {
                            position,
                            normal,
                            texture,
                        })
                    }
                },
            }
        }
    }

    // TODO catch error
    Ok(glium::vertex::VertexBuffer::new(display, &vertex_data).unwrap().into())
}

