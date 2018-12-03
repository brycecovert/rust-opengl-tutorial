extern crate gl;
extern crate sdl2;

use std::fs::File;
extern crate png;
extern crate cgmath;

use cgmath::conv::*;

use std::ffi::{CString};




pub mod render_gl;
fn main() {
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

    let decoder = png::Decoder::new(File::open("tongue-hit_0.png").unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();

    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).unwrap();

    let tex = render_gl::Texture::from_image(&gl, info.width as i32, info.height as i32, buf).unwrap();



    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vertices: Vec<f32> = vec![
              100.0,  100.0,  0.0,   1.0, 1.0, 1.0,   1.0, 1.0,
              100.0,    0.0,  0.0,   1.0, 1.0, 1.0,   1.0, 0.0,
              0.0,      0.0,  0.0,   1.0, 1.0, 1.0,   0.0, 0.0,
              0.0,    100.0,  0.0,   1.0, 1.0, 1.0,   0.0, 1.0
    ];

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
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                std::ptr::null()
        );
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(1,
                                3,
                                gl::FLOAT,
                                gl::FALSE,
                                (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
        );


        gl.EnableVertexAttribArray(2);
        gl.VertexAttribPointer(2,
                               2,
                               gl::FLOAT,
                               gl::FALSE,
                               (8 * std::mem::size_of::<f32>()) as gl::types::GLint,
                               (6 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid
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
                      gl::STATIC_DRAW
        );
    }

    let ortho_matrix = cgmath::ortho(0.0, 900.0, 700.0, 0.0, 0.0, 1.0);
    let x = array4x4(ortho_matrix);
    let y: Vec<f32> = vec![
        x[0][0], x[0][1], x[0][2], x[0][3],
        x[1][0], x[1][1], x[1][2], x[1][3],
        x[2][0], x[2][1], x[2][2], x[2][3],
        x[3][0], x[3][1], x[3][2], x[3][3],
    ];


    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        shader_program.set_used();
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            // gl.Ortho(0.0, 900.0, 0.0, 700.0);
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl.Uniform1i(gl.GetUniformLocation(frag_shader_id, CString::new("texture1").unwrap().as_ptr()), 0);
            gl.UniformMatrix4fv(gl.GetUniformLocation(shader_program.id, CString::new("projectionmatrix").unwrap().as_ptr()), 1, gl::FALSE, y.as_ptr() as *const f32);
            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, tex.id);
            gl.BindVertexArray(vao);
            gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            // gl.DrawArrays(gl::TRIANGLES, 0, 3);
            gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();
    }
}
