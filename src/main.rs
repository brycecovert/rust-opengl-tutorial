extern crate gl;
extern crate sdl2;
extern crate rand;

extern crate png;
extern crate cgmath;

use cgmath::conv::*;

use std::ffi::{CString};
use rand::{thread_rng, Rng};

pub mod texture_region;
pub mod quad;
pub mod vertex;
pub mod render_gl;
pub mod sprite_batch;

use sprite_batch::SpriteBatch;
use texture_region::TextureRegion;
use quad::Quad;
use vertex::Vertex;

struct Entity <'a> {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    texture_region: &'a TextureRegion,
    width: f32,
    height: f32
}

impl<'a> Entity <'a> {
    pub fn to_quad(&self) -> Quad {
        Quad::new(
            Vertex {
                pos: (self.width + self.x, self.y + self.height, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (self.texture_region.u2, self.texture_region.v2)
            },
            Vertex {
                pos: (self.x + self.width, self.y, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (self.texture_region.u2, self.texture_region.v1)
            },
            Vertex {
                pos: (self.x, self.y, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (self.texture_region.u1, self.texture_region.v1)
            },
            Vertex {
                pos: (self.x, self.y + self.height, 0.0),
                color: (1.0, 1.0, 1.0, 1.0),
                uv: (self.texture_region.u1, self.texture_region.v2)
            },
        )
    }
}


fn main() {
    let mut rng = thread_rng();
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let mut window = video_subsystem
        .window("Game", 1280, 760)
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
        gl.Viewport(0, 0, 1280, 760);
        gl.ClearColor(0.1, 0.1, 0.1, 1.0);
    }

    let ortho_matrix = cgmath::ortho(0.0, 1280.0, 760.0, 0.0, 0.0, 1.0);
    let x = array4x4(ortho_matrix);
    let projection: Vec<f32> = x.iter()
        .flat_map(|z| z.iter())
        .cloned()
        .collect();
    let sprite = texture_region::TextureRegion::new_uv(&gl, "tongue-hit_0.png", vert_shader_id, frag_shader_id, shader_program.id, 0.0, 0.0, 1.0, 1.0);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut sprite_batch = SpriteBatch::new(&gl);
    let mut enemy = texture_region::TextureRegion::new(&gl, "enemy.png", vert_shader_id, frag_shader_id, shader_program.id);
    let mut player = Entity {
        width: 119.0,
        height: 134.0,
        texture_region: &sprite,
        x: 500.3,
        y: 300.9,
        vx: 0.0,
        vy: 0.0,
    };
    let mut e: Vec<Entity> = (0..100)
        .map(|_| Entity {
            width: 119.0,
            height: 134.0,
            texture_region: &enemy,
            x: rng.gen_range(0.0, 1280.0),
            y: rng.gen_range(0.0, 760.0),
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
                    window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap();
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), ..} => {
                    window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap();
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
            e.iter_mut().for_each(|d| {
                d.y = d.y + d.vy;
                d.x = d.x + d.vx;
            });
            player.x += player.vx;
            player.y += player.vy;
            e.iter().for_each(|d| sprite_batch.draw(&enemy, &d.to_quad(), &projection));
            sprite_batch.draw(&sprite, &player.to_quad(), &projection);
        }

        sprite_batch.flush();

        window.gl_swap_window();

        use std::{thread, time};

        thread::sleep(time::Duration::from_millis(30));

    }
}
