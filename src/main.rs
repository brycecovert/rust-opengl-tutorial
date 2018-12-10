extern crate gl;
extern crate sdl2;
extern crate rand;

extern crate png;
extern crate cgmath;
extern crate time;
extern crate hashbrown;


use rand::{thread_rng, Rng};
use time::precise_time_s;
// use std::collections::HashMap;

pub mod world;
pub mod texture_region;
pub mod quad;
pub mod vertex;
pub mod render_gl;
pub mod sprite_batch;
pub mod component;
pub mod entity;
pub mod system;

use sprite_batch::SpriteBatch;
use world::World;
use system::render;
use component::{Position, Velocity, Sized, TextureRegioned};



fn update(world: &mut World) {
    let positions = &mut world.positions;
    let velocities = &world.velocities;
    world.entities
        .iter()
        .for_each(|e| {
            let (position, velocity) = (positions.get_mut(e).unwrap(), velocities.get(e).unwrap());
            position.x += velocity.x;
            position.y += velocity.y;
            
        });
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
    let player = world.create_entity(Position::new(500.0, 300.0), Velocity::new(0.0, 0.0), Sized::new(119.0, 134.0), TextureRegioned::new(&sprite));
    {
    (0..1000)
        .for_each(|_| { world.create_entity(Position::new(rng.gen_range(0.0, 1280.0), rng.gen_range(0.0, 1280.0)), Velocity::new(rng.gen_range(-0.1, 0.1), rng.gen_range(-0.1, 0.1)), Sized::new(119.0, 134.0), TextureRegioned::new(&enemy));

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
                    world.velocities.get_mut(&player).unwrap().x = -2.0;
                },
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                    world.velocities.get_mut(&player).unwrap().x = 2.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Up), ..} => {
                    world.velocities.get_mut(&player).unwrap().y = -2.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Down), ..} => {
                    world.velocities.get_mut(&player).unwrap().y = 2.0;
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::H), ..} => {
                    window.set_fullscreen(sdl2::video::FullscreenType::Off).unwrap();
                },

                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::F), ..} => {
                    window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap();
                },

                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Left), ..} => {
                    world.velocities.get_mut(&player).unwrap().x = 0.0;
                },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Up), ..} => {
                    world.velocities.get_mut(&player).unwrap().y = 0.0;
                },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Right), ..} => {
                    world.velocities.get_mut(&player).unwrap().x = 0.0;
                },
                sdl2::event::Event::KeyUp { keycode: Some(sdl2::keyboard::Keycode::Down), ..} => {
                    world.velocities.get_mut(&player).unwrap().y = 0.0;
                },
                _ => {}
            }
        }

        while accumulator >= update_time {
            update(&mut world);
            accumulator -= update_time;
        }

        render::update(&gl, &world, &mut sprite_batch);
        window.gl_swap_window();
    }
}
