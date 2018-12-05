use std::fs::File;
use render_gl;
use quad::Quad;
use vertex::Vertex;

pub struct TextureRegion {
    pub gl: gl::Gl,
    pub texture: render_gl::Texture,
    pub shader: u32,
    pub frag_shader_id: u32,
    pub vert_shader_id: u32,
    pub quad: Quad,
    pub vertices: Vec<f32>,
}


impl TextureRegion {
    pub fn new_uv(gl: &gl::Gl, image: &str, vert_shader_id: u32, frag_shader_id: u32, shader: u32, u1: f32, v1: f32, u2: f32, v2: f32) -> TextureRegion {
        let decoder = png::Decoder::new(File::open(image).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let tex = render_gl::Texture::from_image(&gl, info.width as i32, info.height as i32, buf).unwrap();

        let quad = Quad(
            Vertex {
                pos: (info.width as f32, info.height as f32, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (u2, v2)
            },
            Vertex {
                pos: (info.width as f32, 0.0, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (u2, v1)
            },
            Vertex {
                pos: (0.0, 0.0, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (u1, v1)
            },
            Vertex {
                pos: (0.0, info.height as f32, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (u1, v2)
            },
        );


        let vertices: Vec<f32> = quad.to_vertex_data();
        println!("{:?}", vertices);
        let v_d = quad.to_vertex_data();

        TextureRegion { gl: gl.clone(), frag_shader_id, vert_shader_id, texture: tex, shader, quad, vertices: v_d }
    }


    pub fn new(gl: &gl::Gl, image: &str, vert_shader_id: u32, frag_shader_id: u32, shader: u32) -> TextureRegion {
        let decoder = png::Decoder::new(File::open(image).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let tex = render_gl::Texture::from_image(&gl, info.width as i32, info.height as i32, buf).unwrap();

        let quad = Quad(
            Vertex {
                pos: (info.width as f32, info.height as f32, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (1.0, 1.0)
            },
            Vertex {
                pos: (info.width as f32, 0.0, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (1.0, 0.0)
            },
            Vertex {
                pos: (0.0, 0.0, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (0.0, 0.0)
            },
            Vertex {
                pos: (0.0, info.height as f32, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (0.0, 1.0)
            },
        );


        let vertices: Vec<f32> = quad.to_vertex_data();
        println!("{:?}", vertices);
        let v_d = quad.to_vertex_data();

        TextureRegion { gl: gl.clone(), frag_shader_id, vert_shader_id, texture: tex, shader, quad, vertices: v_d }
    }

}
