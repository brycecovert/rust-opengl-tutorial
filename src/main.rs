extern crate gl;
extern crate sdl2;
extern crate rand;

use std::fs::File;
extern crate png;
extern crate cgmath;

use cgmath::conv::*;

use std::ffi::{CString};
use rand::{thread_rng, Rng};


struct Sprite {
    pub gl: gl::Gl,
    pub texture: render_gl::Texture,
    pub shader: u32,
    pub frag_shader_id: u32,
    pub vert_shader_id: u32,
    pub quad: Quad,
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub vertices: Vec<f32>,
}

#[derive(Debug, Clone)]
struct Vertex {
    pub pos: (f32, f32, f32),
    pub color: (f32, f32, f32, f32),
    pub uv: (f32, f32)
}

#[derive(Debug, Clone)]
struct Quad (Vertex, Vertex, Vertex, Vertex);

impl Quad {
    fn to_vertex_data(&self) -> Vec<f32> {
        vec![
            self.0.pos.0, self.0.pos.1, self.0.pos.2,
            self.0.color.0, self.0.color.1, self.0.color.2, self.0.color.3,
            self.0.uv.0, self.0.uv.1,

            self.1.pos.0, self.1.pos.1, self.1.pos.2,
            self.1.color.0, self.1.color.1, self.1.color.2, self.1.color.3,
            self.1.uv.0, self.1.uv.1,

            self.2.pos.0, self.2.pos.1, self.2.pos.2,
            self.2.color.0, self.2.color.1, self.2.color.2, self.2.color.3,
            self.2.uv.0, self.2.uv.1,

            self.3.pos.0, self.3.pos.1, self.3.pos.2,
            self.3.color.0, self.3.color.1, self.3.color.2, self.3.color.3,
            self.3.uv.0, self.3.uv.1,
        ]
    }

    fn add(&self, x: f32,  y: f32) -> Quad {
        let mut n = self.clone();
        n.0.pos.0 += x;
        n.1.pos.0 += x;
        n.2.pos.0 += x;
        n.3.pos.0 += x;

        n.0.pos.1 += y;
        n.1.pos.1 += y;
        n.2.pos.1 += y;
        n.3.pos.1 += y;
        n
    }
}

impl Sprite {
    pub fn new(gl: &gl::Gl, image: &str, vert_shader_id: u32, frag_shader_id: u32, shader: u32) -> Sprite {
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

        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(gl::ARRAY_BUFFER,
                          (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                          vertices.as_ptr() as *const gl::types::GLvoid,
                          gl::STATIC_DRAW,
            );
            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
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


            gl.BindBuffer(gl::ARRAY_BUFFER, 0);
            gl.BindVertexArray(0);
        }

        let indices: Vec<u32> = vec![
            0, 1, 3,
            1, 2, 3
        ];

        let mut ebo: gl::types::GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut ebo);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl.BufferData(gl::ELEMENT_ARRAY_BUFFER,
                          (indices.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                          indices.as_ptr() as *const gl::types::GLvoid,
                          gl::STATIC_DRAW);
        }

        let v_d = quad.to_vertex_data();

            Sprite { gl: gl.clone(), frag_shader_id, vert_shader_id, texture: tex, vbo, vao, ebo, shader, quad, vertices: v_d }
    }

    pub fn draw(&self, projection: &Vec<f32>, x: f32, y: f32) {
        let _vertices: Vec<f32> = self.quad.add(x, y).to_vertex_data();
        unsafe {
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            self.gl.BufferSubData(gl::ARRAY_BUFFER,
                                  0,
                               (_vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                               _vertices.as_ptr() as *const gl::types::GLvoid,
                               
            );
            self.gl.BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        unsafe {
            self.gl.Uniform1i(self.gl.GetUniformLocation(self.frag_shader_id, CString::new("texture1").unwrap().as_ptr()), 0);
            self.gl.UniformMatrix4fv(self.gl.GetUniformLocation(self.shader, CString::new("projectionmatrix").unwrap().as_ptr()), 1, gl::FALSE, projection.as_ptr() as *const f32);
            self.gl.ActiveTexture(gl::TEXTURE0);
            self.gl.BindTexture(gl::TEXTURE_2D, self.texture.id);
            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            self.gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
        
    }
    
}



pub mod render_gl;
fn main() {
    let mut rng = thread_rng();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    let vert_shader = render_gl::Shader::from_vert_source(
        &gl,
        include_str!("triangle.vert")
    ).expect("There should be no problem vert");

    let vert_shader_id = vert_shader.id;

    let frag_shader = render_gl::Shader::from_frag_source(
        &gl,
        include_str!("triangle.frag")
    ).expect("There should be no problem frag");

    let frag_shader_id = frag_shader.id;
    let shader_program = render_gl::Program::from_shaders(
        &gl,
        &[vert_shader, frag_shader]
    ).unwrap();

    shader_program.set_used();

    println!("shader id: {}", shader_program.id);
    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let ortho_matrix = cgmath::ortho(0.0, 900.0, 700.0, 0.0, 0.0, 1.0);
    let x = array4x4(ortho_matrix);
    let projection: Vec<f32> = x.iter()
        .flat_map(|z| z.iter())
        .cloned()
        .collect();
    let sprite = Sprite::new(&gl, "tongue-hit_0.png", vert_shader_id, frag_shader_id, shader_program.id);
    let mut locations_x: Vec<f32> = vec![0.0];
    let mut locations_y: Vec<f32> = vec![0.0];
    let mut event_pump = sdl.event_pump().unwrap();
    for i in 0..500 {
        locations_x.push(rng.gen_range(0.0, 900.0));
        locations_y.push(rng.gen_range(0.0, 700.0));
    }

    'main: loop {
        let begin = time::Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        shader_program.set_used();
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            locations_x.iter().zip(locations_y.iter()).for_each(|(x, y)| sprite.draw(&projection, *x, *y));
        }

        window.gl_swap_window();

        use std::{thread, time};
        use std::ops::Sub;
        let end = time::Instant::now();
        if rng.gen_range(0, 60) == 0 {
            println!("millis {}", end.sub(begin).subsec_millis())
        }

        thread::sleep(time::Duration::from_millis(10));

    }
}
