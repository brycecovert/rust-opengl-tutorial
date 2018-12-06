use std::fs::File;
use render_gl;

pub struct TextureRegion {
    pub gl: gl::Gl,
    pub texture: render_gl::Texture,
    pub shader: u32,
    pub frag_shader_id: u32,
    pub vert_shader_id: u32,
    pub u1: f32,
    pub u2: f32,
    pub v1: f32,
    pub v2: f32,
}


impl TextureRegion {
    pub fn new_uv(gl: &gl::Gl, image: &str, vert_shader_id: u32, frag_shader_id: u32, shader: u32, u1: f32, v1: f32, u2: f32, v2: f32) -> TextureRegion {
        let decoder = png::Decoder::new(File::open(image).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let tex = render_gl::Texture::from_image(&gl, info.width as i32, info.height as i32, buf).unwrap();


        TextureRegion { gl: gl.clone(), frag_shader_id, vert_shader_id, texture: tex, shader, u1, u2, v1, v2 }
    }


    pub fn new(gl: &gl::Gl, image: &str, vert_shader_id: u32, frag_shader_id: u32, shader: u32) -> TextureRegion {
        TextureRegion::new_uv(gl, image, vert_shader_id, frag_shader_id, shader, 0.0, 0.0, 1.0, 1.0)
    }

}
