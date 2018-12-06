use texture_region::TextureRegion;
use quad::Quad;
use render_gl::Program;
use cgmath::Matrix4;
use cgmath::conv::array4x4;
use std::ffi::{CString};

pub struct SpriteBatch<'a> {
    gl: gl::Gl,
    shader_program: &'a Program,
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<u32>,
    count: u32,
    current_texture: u32,
    projection: Vec<f32>

}

impl <'a> SpriteBatch<'a> {
    pub fn new(gl: &gl::Gl, shader_program: &'a Program, projection: Matrix4<f32>) -> SpriteBatch<'a> {
        let projection: Vec<f32> = array4x4(projection).iter()
            .flat_map(|z| z.iter())
            .cloned()
            .collect();

        let mut result = SpriteBatch { gl: gl.clone(), vao: 0, vbo: 0, ebo: 0, vertex_buffer: Vec::with_capacity(400000), index_buffer: Vec::with_capacity(400000), count: 0, current_texture: 0, shader_program, projection};
        unsafe {
            gl.GenBuffers(1, &mut result.vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, result.vbo);
        }

        unsafe {
            gl.GenVertexArrays(1, &mut result.vao);
            gl.BindVertexArray(result.vao);
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(0,
                                   4,
                                   gl::FLOAT,
                                   gl::FALSE,
                                   (9 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                   std::ptr::null()
            );
            gl.EnableVertexAttribArray(1);
            gl.VertexAttribPointer(1,
                                   3,
                                   gl::FLOAT,
                                   gl::FALSE,
                                   (9 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                   (4 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
            );


            gl.EnableVertexAttribArray(2);
            gl.VertexAttribPointer(2,
                                   2,
                                   gl::FLOAT,
                                   gl::FALSE,
                                   (9 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                   (7 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
            );
        }

        unsafe {
            gl.GenBuffers(1, &mut result.ebo);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, result.ebo);
        }
        result

            
    }

    pub fn draw(&mut self, sprite: &TextureRegion, quad: &Quad) {
        let mut _vertices: Vec<f32> = quad.to_vertex_data();
        if self.current_texture != sprite.texture.id {
            self.flush();
            unsafe {
                self.gl.ActiveTexture(gl::TEXTURE0);
                self.gl.BindTexture(gl::TEXTURE_2D, sprite.texture.id);
            }
            
        }
        self.current_texture = sprite.texture.id;
        self.vertex_buffer.append(&mut _vertices);
        self.index_buffer.append(&mut vec![
            self.count * 4 + 0, self.count * 4 + 1, self.count * 4 + 3,
            self.count * 4 + 1, self.count * 4 + 2, self.count * 4 + 3,
        ]);
        self.count += 1;
    }

    pub fn flush(&mut self) {
        self.count = 0;
        self.shader_program.set_used();
        unsafe {
            self.gl.UniformMatrix4fv(self.gl.GetUniformLocation(self.shader_program.id, CString::new("projectionmatrix").unwrap().as_ptr()), 1, gl::FALSE, self.projection.as_ptr() as *const f32);
            self.gl.Uniform1i(self.gl.GetUniformLocation(self.shader_program.id, CString::new("texture1").unwrap().as_ptr()), 0);
            self.gl.Enable(gl::BLEND);
            self.gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            self.gl.BufferData (gl::ARRAY_BUFFER,
                               (self.vertex_buffer.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                               self.vertex_buffer.as_ptr() as *const gl::types::GLvoid,
                                gl::STATIC_DRAW
                               
            );
        }
        unsafe {
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl.BufferData(gl::ELEMENT_ARRAY_BUFFER,
                          (self.index_buffer.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                          self.index_buffer.as_ptr() as *const gl::types::GLvoid,
                          gl::STATIC_DRAW);
        }
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl.DrawElements(gl::TRIANGLES, self.index_buffer.len() as i32, gl::UNSIGNED_INT, std::ptr::null());
        }

        self.vertex_buffer.clear();
        self.index_buffer.clear();
    }
}

