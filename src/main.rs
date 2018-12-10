extern crate gl;
extern crate sdl2;
extern crate rand;

extern crate png;
extern crate cgmath;
extern crate time;
extern crate hashbrown;
use hashbrown::HashMap;


use rand::{thread_rng, Rng};
use time::precise_time_s;
// use std::collections::HashMap;

pub mod texture_region;
pub mod quad;
pub mod vertex;
pub mod render_gl;
pub mod sprite_batch;

use sprite_batch::SpriteBatch;
use texture_region::TextureRegion;
use quad::Quad;
use vertex::Vertex;

trait Component {
}

pub struct Position(f32, f32);
pub struct Velocity(f32, f32);
pub struct Sized(f32, f32);
pub struct TextureRegioned<'a> (&'a TextureRegion);

impl Component for Position {}
impl Component for Velocity {}
impl Component for Sized {}
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Entity (u32);

struct World<'a> {
    last_id: u32,
    entities: Vec<Entity>,
    positions: HashMap<Entity, Position>,
    velocities: HashMap<Entity, Velocity>,
    sizes: HashMap<Entity, Sized>,
    texture_regioned: HashMap<Entity, TextureRegioned<'a>>,
}

impl <'a> World <'a> {
    fn new() -> World<'a> {
        World {
            last_id: 0,
            entities: Vec::new(),
            positions: HashMap::new(),
            velocities: HashMap::new(),
            sizes: HashMap::new(),
            texture_regioned: HashMap::new(),
        }
    }
    fn create_entity(&mut self, p: Position, v: Velocity, s: Sized, t: TextureRegioned<'a>)  -> Entity {
        self.last_id += 1;
        let id = self.last_id;;
        let e = Entity(id);
        self.positions.insert(e, p);
        self.velocities.insert(e, v);
        self.sizes.insert(e, s);
        self.texture_regioned.insert(e, t);
        self.entities.push(e);
        e
    }
}

pub fn to_quad(p: &Position, s: &Sized, t: &TextureRegion) -> Quad {
    Quad::new(
        Vertex {
            pos: (s.0 + p.0, p.1 + s.1, 0.0),
            color: (1.0, 1.0, 1.0, 1.0),
            uv: (t.u2, t.v2)
        },
        Vertex {
            pos: (p.0 + s.0, p.1, 0.0),
            color: (1.0, 1.0, 1.0, 1.0),
            uv: (t.u2, t.v1)
        },
        Vertex {
            pos: (p.0, p.1, 0.0),
            color: (1.0, 1.0, 1.0, 1.0),
            uv: (t.u1, t.v1)
        },
        Vertex {
            pos: (p.0, p.1 + s.1, 0.0),
            color: (1.0, 1.0, 1.0, 1.0),
            uv: (t.u1, t.v2)
        },
    )
}

fn update(world: &mut World) {
    let positions = &mut world.positions;
    let velocities = &world.velocities;
    world.entities
        .iter()
        .for_each(|e| {
            let (position, velocity) = (positions.get_mut(e).unwrap(), velocities.get(e).unwrap());
            position.0 += velocity.0;
            position.1 += velocity.1;
            
        });
}


fn render(gl: &gl::Gl, world: &World, sprite_batch: &mut SpriteBatch) {
    unsafe {
        gl.Clear(gl::COLOR_BUFFER_BIT);
        world.entities
            .iter()
            .for_each(|e| {
                let (position, size, texture_regioned) = (world.positions.get(e).unwrap(), world.sizes.get(e).unwrap(), world.texture_regioned.get(e).unwrap());
                sprite_batch.draw(&texture_regioned.0, &to_quad(&position, &size, &texture_regioned.0));

                
            });
    }

    sprite_batch.flush();
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

    let frag_shader = render_gl::Shader::from_frag_source(
        &gl,
        include_str!("triangle.frag")
    ).expect("There should be no problem frag");

    let shader_program = render_gl::Program::from_shaders(
        &gl,
        &[vert_shader, frag_shader]
    ).unwrap();

    println!("shader id: {}", shader_program.id);
    unsafe {
        gl.Viewport(0, 0, 1280, 760);
        gl.ClearColor(0.1, 0.1, 0.1, 1.0);
    }

    let ortho_matrix = cgmath::ortho(0.0, 1280.0, 760.0, 0.0, 0.0, 1.0);
    let sprite = texture_region::TextureRegion::new_uv(&gl, "tongue-hit_0.png", 0.0, 0.0, 1.0, 1.0);
    let mut event_pump = sdl.event_pump().unwrap();
    let mut enemy = texture_region::TextureRegion::new(&gl, "enemy.png");
    let mut sprite_batch = SpriteBatch::new(&gl, &shader_program, ortho_matrix);
    let mut world = World::new();
;
    let player = world.create_entity(Position(500.0, 300.0), Velocity(0.0, 0.0), Sized(119.0, 134.0), TextureRegioned(&sprite));
    {
    (0..1000)
        .for_each(|_| { world.create_entity(Position(rng.gen_range(0.0, 1280.0), rng.gen_range(0.0, 1280.0)), Velocity(rng.gen_range(-0.1, 0.1), rng.gen_range(-0.1, 0.1)), Sized(119.0, 134.0), TextureRegioned(&enemy));

        });
}
    let update_time: f64 = 0.01;
    let mut current_time: f64 = precise_time_s();
    let mut accumulator: f64 = 0.0;
    let mut s: i32 = current_time as i32;
    let mut frame_count = 0;
    'main: loop {
        frame_count += 1;
        let start_time: f64 = precise_time_s();
        let last_frame_time = start_time - current_time;
        current_time = start_time;
        if current_time as i32 != s {
            s = current_time as i32;
            println!("FPS {}", frame_count);
            frame_count = 0;
        }

        accumulator += last_frame_time;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { ..  } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Q), ..} => break 'main,
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Left), ..} => {
                    world.velocities.get_mut(&player).unwrap().0 = -2.0;
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                    world.velocities.get_mut(&player).unwrap().0 = 2.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), ..} => {
                    world.velocities.get_mut(&player).unwrap().1 = -2.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), ..} => {
                    world.velocities.get_mut(&player).unwrap().1 = 2.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::H), ..} => {
                    window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap();
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), ..} => {
                    window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap();
                },

                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Left), ..} => {
                    world.velocities.get_mut(&player).unwrap().0 = 0.0;
                },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Up), ..} => {
                    world.velocities.get_mut(&player).unwrap().1 = 0.0;
                },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                    world.velocities.get_mut(&player).unwrap().0 = 0.0;
                },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Down), ..} => {
                    world.velocities.get_mut(&player).unwrap().1 = 0.0;
                },
                _ => {}
            }
        }

        while accumulator >= update_time {
            update(&mut world);
            accumulator -= update_time;
        }

        render(&gl, &world, &mut sprite_batch);
        window.gl_swap_window();
    }
}
