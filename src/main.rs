
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

//#![windows_subsystem = "windows"]

mod controller;
mod spritebatch;

use glium::{glutin, Surface};
use glium::index::PrimitiveType;
use cgmath::SquareMatrix;
use controller::Controller;
use spritebatch::SpriteBatch;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // compiling shaders and linking them together
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 world;
                uniform mat4 projection;
                uniform mat4 view;
                in vec2 position;
                in vec2 tex_coord;
                out vec2 v_texcoord;
                void main() {
                    mat4 wvp = world * view * projection;
                    gl_Position = wvp * vec4(position, 0.0, 1.0);
                    v_texcoord = tex_coord;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_texcoord;
                out vec4 f_color;
                void main() {
                    vec4 tex_color = texture(tex, v_texcoord);
                    f_color = tex_color;
                }
            "
        },
    ).unwrap();

    let mut sprite_batch = SpriteBatch::new();
    sprite_batch.add(16.0,16.0);
    let path = "small-mario.png";
    let path = std::path::Path::new(&path);
    let image = image::open(&path).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    let opengl_texture = glium::Texture2d::new(&display, image).unwrap();
   
    let draw = |controller:&Controller, x : f32, y : f32| {
        let x = if controller.left { x - 0.001f32 } else { if controller.right { x + 0.001f32 } else { x } };
        let y = if controller.up { y + 0.001f32 } else { if controller.down { y - 0.001f32 } else { y } };
        let vertices = sprite_batch.quads.iter().flat_map(|s| s).map(|s| s.clone()).collect::<Vec<_>>();
        let vertex_buffer = glium::VertexBuffer::new(&display,&vertices).unwrap();
        let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList,
                                               &sprite_batch.indices).unwrap();

        let world = cgmath::Matrix4::from_translation(cgmath::Vector3::new(x, y, 0.0f32)); 
        let view = cgmath::Matrix4::identity();
        let projection = cgmath::ortho(0.0f32, 320.0f32, 240.0f32, 0.0f32, 0.0f32, 100.0f32);
        // building the uniforms
        let uniforms = uniform! {
            world: Into::<[[f32;4];4]>::into(world),
            view: Into::<[[f32;4];4]>::into(view),
            projection: Into::<[[f32;4];4]>::into(projection),
            tex: &opengl_texture,
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
        (x, y)
    };

    let mut controller = Controller::new();
    let mut x = 0.0f32; 
    let mut y = 0.0f32;


    // Draw the triangle to the screen.
    let (x1,y1) = draw(&controller, x, y);
    x = x1;
    y = y1;

    // the main loop
    let mut closed = false;
    while !closed {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => 
                    match event {
                        // Break from the main loop when the window is closed.
                        glutin::WindowEvent::Closed => closed = true,
                        // Redraw the triangle when the window is resized.
                        //glutin::WindowEvent::Resized(..) => draw(rot),
                        glutin::WindowEvent::KeyboardInput { input, .. }  => {
                            if input.virtual_keycode == Some(glutin::VirtualKeyCode::Left) {
                                controller.left = input.state == glutin::ElementState::Pressed;
                            }
                            if input.virtual_keycode == Some(glutin::VirtualKeyCode::Right) {
                                controller.right = input.state == glutin::ElementState::Pressed;
                            }
                            if input.virtual_keycode == Some(glutin::VirtualKeyCode::Up) {
                                controller.up = input.state == glutin::ElementState::Pressed;
                            }
                            if input.virtual_keycode == Some(glutin::VirtualKeyCode::Down) {
                                controller.down = input.state == glutin::ElementState::Pressed;
                            }
                        },
                        _ => (),
                    },
                _ => (),
            }
        });
        let (x1,y1) = draw(&controller, x, y);
        x = x1;
        y = y1;
    }
}
