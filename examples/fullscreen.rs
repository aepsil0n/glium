#![feature(plugin)]
#![plugin(glium_macros)]

extern crate glutin;

#[macro_use]
extern crate glium;

#[cfg(feature = "image")]
extern crate image;

#[cfg(feature = "image")]
use std::old_io::BufReader;

#[cfg(feature = "image")]
use glium::{DisplayBuild, Surface};

mod support;

#[cfg(not(feature = "image"))]
fn main() {
    println!("This example requires the `image` feature to be enabled");
}

#[cfg(feature = "image")]
fn main() {
    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new().build_glium().unwrap();

    // building a texture with "OpenGL" drawn on it
    let image = image::load(BufReader::new(include_bytes!("../tests/fixture/opengl.png")),
        image::PNG).unwrap();
    let opengl_texture = glium::texture::CompressedTexture2d::new(&display, image);

    // building the vertex buffer, which contains all the vertices that we will draw
    let vertex_buffer = {
        #[vertex_format]
        #[derive(Copy)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        glium::VertexBuffer::new(&display, 
            vec![
                Vertex { position: [-1.0, -1.0], tex_coords: [0.0, 0.0] },
                Vertex { position: [-1.0,  1.0], tex_coords: [0.0, 1.0] },
                Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] },
                Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] }
            ]
        )
    };

    // building the index buffer
    let index_buffer = glium::IndexBuffer::new(&display,
        glium::index::TriangleStrip(vec![1 as u16, 2, 0, 3]));

    // compiling shaders and linking them together
    let program = glium::Program::from_source(&display, r"
        #version 110

        uniform mat4 matrix;

        attribute vec2 position;
        attribute vec2 tex_coords;

        varying vec2 v_tex_coords;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
            v_tex_coords = tex_coords;
        }
    ", r"
        #version 110
        uniform sampler2D texture;
        varying vec2 v_tex_coords;

        void main() {
            gl_FragColor = texture2D(texture, v_tex_coords);
        }
    ", None).unwrap();


    let mut fullscreen = false;

    println!("Press Enter to switch fullscreen mode");

    support::start_loop(|| {
        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniform! { 
                matrix: [
                    [0.5, 0.0, 0.0, 0.0],
                    [0.0, 0.5, 0.0, 0.0],
                    [0.0, 0.0, 0.5, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32]
                ],
                texture: &opengl_texture
            }, &std::default::Default::default()).unwrap();
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _,
                                             Some(glutin::VirtualKeyCode::Return)) =>
                {
                    if fullscreen {
                        glutin::WindowBuilder::new().rebuild_glium(&display).unwrap();
                        fullscreen = false;

                    } else {
                        glutin::WindowBuilder::new().with_fullscreen(glutin::get_primary_monitor())
                                                    .rebuild_glium(&display).unwrap();
                        fullscreen = true;
                    }
                },
                _ => ()
            }
        }

        support::Action::Continue
    });
}
