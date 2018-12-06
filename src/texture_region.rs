use std::fs::File;
use render_gl;

pub struct TextureRegion {
    pub gl: gl::Gl,
    pub texture: render_gl::Texture,
    pub u1: f32,
    pub u2: f32,
    pub v1: f32,
    pub v2: f32,
}


impl TextureRegion {
    pub fn new_uv(gl: &gl::Gl, image: &str, u1: f32, v1: f32, u2: f32, v2: f32) -> TextureRegion {
        let decoder = png::Decoder::new(File::open(image).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf).unwrap();

        let tex = render_gl::Texture::from_image(&gl, info.width as i32, info.height as i32, buf).unwrap();


        TextureRegion { gl: gl.clone(), texture: tex, u1, u2, v1, v2 }
    }


    pub fn new(gl: &gl::Gl, image: &str) -> TextureRegion {
        TextureRegion::new_uv(gl, image, 0.0, 0.0, 1.0, 1.0)
    }

}
