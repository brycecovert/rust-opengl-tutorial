extern crate gl;
extern crate sdl2;
extern crate rand;

use std::fs::File;
extern crate png;
extern crate cgmath;

use cgmath::conv::*;

use std::ffi::{CString};
use rand::{thread_rng, Rng};


struct TextureRegion {
    pub gl: gl::Gl,
    pub texture: render_gl::Texture,
    pub shader: u32,
    pub frag_shader_id: u32,
    pub vert_shader_id: u32,
    pub quad: Quad,
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

struct SpriteBatch {
    gl: gl::Gl,
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    vertex_buffer: Vec<f32>,
    index_buffer: Vec<u32>,
    count: u32
}

impl SpriteBatch {
    pub fn new(gl: &gl::Gl) -> SpriteBatch {
        let mut result = SpriteBatch { gl: gl.clone(), vao: 0, vbo: 0, ebo: 0, vertex_buffer: Vec::with_capacity(400000), index_buffer: Vec::with_capacity(400000), count: 0};
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

    pub fn draw(&mut self, sprite: &TextureRegion, projection: &Vec<f32>, x: f32, y: f32) {
        let mut _vertices: Vec<f32> = sprite.quad.add(x, y).to_vertex_data();
        self.vertex_buffer.append(&mut _vertices);
        self.index_buffer.append(&mut vec![
            self.count * 4 + 0, self.count * 4 + 1, self.count * 4 + 3,
            self.count * 4 + 1, self.count * 4 + 2, self.count * 4 + 3,
        ]);
        self.count += 1;
    }

    pub fn flush(&mut self) {
        self.count = 0;
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


struct Entity {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32
}


pub mod render_gl;
fn main() {
    let mut rng = thread_rng();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let mut window = video_subsystem
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
    let sprite = TextureRegion::new_uv(&gl, "tongue-hit_0.png", vert_shader_id, frag_shader_id, shader_program.id, 0.0, 0.0, 0.5, 0.7);
    let sprite2 = TextureRegion::new_uv(&gl, "tongue-hit_0.png", vert_shader_id, frag_shader_id, shader_program.id, 0.0, 0.0, 1.0, 1.0);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut sprite_batch = SpriteBatch::new(&gl);
    let mut player = Entity {
        x: 500.3,
        y: 300.9,
        vx: 0.0,
        vy: 0.0,
    };
    let mut e: Vec<Entity> = (0..1)
        .map(|_| Entity {
            x: rng.gen_range(0.0, 900.0),
            y: rng.gen_range(0.0, 700.0),
            vx: rng.gen_range(-4.0, 4.0),
            vy: rng.gen_range(-4.0, 4.0),
        })
        .collect();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { ..  } => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), ..} => {
                    player.vx = -2.0;
                    player.vy = 0.0;
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                    player.vx = 2.0;
                    player.vy = 0.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), ..} => {
                    player.vx = 0.0;
                    player.vy = -2.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), ..} => {
                    player.vx = 0.0;
                    player.vy = 2.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::H), ..} => {
                    window.set_fullscreen(sdl2::video::FullscreenType::Off);
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), ..} => {
                    window.set_fullscreen(sdl2::video::FullscreenType::Desktop);
                },
                _ => {}
            }
        }

        shader_program.set_used();
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl.Uniform1i(gl.GetUniformLocation(shader_program.id, CString::new("texture1").unwrap().as_ptr()), 0);
            gl.UniformMatrix4fv(gl.GetUniformLocation(shader_program.id, CString::new("projectionmatrix").unwrap().as_ptr()), 1, gl::FALSE, projection.as_ptr() as *const f32);
            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, sprite.texture.id);
            e.iter_mut().for_each(|d| {
                d.y = d.y + d.vy;
                d.x = d.x + d.vx;
            });
            player.x += player.vx;
            player.y += player.vy;
            e.iter().for_each(|d| sprite_batch.draw(&sprite2, &projection, d.x, d.y));
            sprite_batch.draw(&sprite, &projection, player.x, player.y);
        }

        sprite_batch.flush();

        window.gl_swap_window();

        use std::{thread, time};

        thread::sleep(time::Duration::from_millis(20));

    }
}
