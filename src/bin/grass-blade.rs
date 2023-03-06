#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

const WINDOW_TITLE: &str = "Triangle: Draw Arrays Cleaned Up";

use beryllium::*;
use core::{
  convert::{TryFrom, TryInto},
  mem::{size_of, size_of_val},
};
use grass::{
  Buffer, BufferType, Shader, ShaderProgram, ShaderType, VertexArray,
};
use grass_opengl as grass;
use ogl33::*;

type Vertex = [f32; 3];

const VERTICES: [Vertex; 3] =
[[-0.1, -0.5, 0.0], [0.1, -0.5, 0.0], [0.0, 0.5, 0.0]];

const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;

  out vec4 vertex_color;

  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
    vertex_color = vec4(0.69 + 0.22*pos.y*2, 0.81 + 0.19*pos.y*2, 0.19- 0.03*pos.y*2, 1 - 0.4 * pos.y*2);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  in vec4 vertex_color;

  out vec4 final_color;

  void main() {
    final_color = vertex_color;
  }
"#;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't start SDL");
  sdl.gl_set_attribute(SdlGlAttr::MajorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::MinorVersion, 3).unwrap();
  sdl.gl_set_attribute(SdlGlAttr::Profile, GlProfile::Core).unwrap();
  #[cfg(target_os = "macos")]
  {
    sdl
      .gl_set_attribute(SdlGlAttr::Flags, ContextFlag::ForwardCompatible)
      .unwrap();
  }

  let win = sdl
    .create_gl_window(
      WINDOW_TITLE,
      WindowPosition::Centered,
      800,
      600,
      WindowFlags::Shown,
    )
    .expect("couldn't make a window and context");
  win.set_swap_interval(SwapInterval::Vsync);

  unsafe {
    load_gl_with(|f_name| win.get_proc_address(f_name));
  }

  grass::clear_color(0.85, 0.92, 1.0, 0.5);

  let vao = VertexArray::new().expect("Couldn't make a VAO");
  vao.bind();

  let vbo = Buffer::new().expect("Couldn't make a VBO");
  vbo.bind(BufferType::Array);
  grass::buffer_data(
    BufferType::Array,
    bytemuck::cast_slice(&VERTICES),
    GL_STATIC_DRAW,
  );

  unsafe {
    glVertexAttribPointer(
      0,
      3,
      GL_FLOAT,
      GL_FALSE,
      size_of::<Vertex>().try_into().unwrap(),
      0 as *const _,
    );
    glEnableVertexAttribArray(0);
  }

  let shader_program =
    ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER).unwrap();
  shader_program.use_program();

  'main_loop: loop {
    // handle events this frame
    while let Some(event) = sdl.poll_events().and_then(Result::ok) {
      match event {
        Event::Quit(_) => break 'main_loop,
        _ => (),
      }
    }
    // now the events are clear.

    // here's where we could change the world state if we had some.

    // and then draw!
    unsafe {
      glClear(GL_COLOR_BUFFER_BIT);
      glDrawArrays(GL_TRIANGLES, 0, 3);
    }
    win.swap_window();
  }
}
