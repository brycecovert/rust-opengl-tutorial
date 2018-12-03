extern crate gl;
extern crate sdl2;

use std::ffi::{CString};

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}


pub struct Shader {
    pub gl: gl::Gl,
    pub id: gl::types::GLuint,
}

impl Shader {

    fn get_shader_log(&self, id: u32) -> String {
        let mut len: gl::types::GLint = 0;
        unsafe {
            self.gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        
        let log = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            self.gl.GetShaderInfoLog(id, len, std::ptr::null_mut(), log.as_ptr() as *mut gl::types::GLchar)
        }

        return log.to_string_lossy().into_owned();
    }

    fn from_source(gl: &gl::Gl, source: &str, kind: gl::types::GLuint) -> Result<Shader, String> {
        let id = unsafe { gl.CreateShader(kind) };
        let mut success: gl::types::GLint = 1;
        let shader = Shader { id, gl: gl.clone() };
        unsafe {
            gl.ShaderSource(id, 1, &CString::new(source).unwrap().as_ptr(), std::ptr::null());
            gl.CompileShader(id);
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                return Err(shader.get_shader_log(id))
            }
        }
        Ok(shader)
    }

    pub fn from_vert_source(gl: &gl::Gl, source: &str) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }
    

    pub fn from_frag_source(gl: &gl::Gl, source: &str) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}




pub struct Program {
    pub gl: gl::Gl,
    pub id: gl::types::GLuint,
}

impl Program {
    fn get_program_log(&self, id: u32) -> String {
        let mut len: gl::types::GLint = 0;
        unsafe {
            self.gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        println!("Error length, {}", len);
        
        let log = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            self.gl.GetProgramInfoLog(id, len, std::ptr::null_mut(), log.as_ptr() as *mut gl::types::GLchar)
        }

        return log.to_string_lossy().into_owned();
    }

    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };
        let program = Program { id: program_id, gl: gl.clone() };
        println!("creating program, {}", program_id);
        for shader in shaders {
            unsafe { gl.AttachShader(program_id, shader.id); }
        }

        unsafe { gl.LinkProgram(program_id); }
        
        for shader in shaders {
            unsafe { gl.DetachShader(program_id, shader.id); }
        }

        let mut success: gl::types::GLint = 0;

        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            println!("This error");

            return Err(program.get_program_log(program_id))
        }

        Ok(program)
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Texture {
    pub gl: gl::Gl,
    pub id: gl::types::GLuint,
}

impl Texture {
    pub fn from_image(gl: &gl::Gl, width: i32, height: i32, data: Vec<u8>) -> Result<Texture, String> {
        let mut tex: gl::types::GLuint = 0;

        unsafe {
            gl.GenTextures(1, &mut tex);
            gl.BindTexture(gl::TEXTURE_2D, tex);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as gl::types::GLint, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, data.as_ptr() as *const gl::types::GLvoid);
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }
        Ok(Texture {id: tex, gl: gl.clone()})
   }
}
