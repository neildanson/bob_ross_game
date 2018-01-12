
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

//#![windows_subsystem = "windows"]
mod controller;
mod sprite;
mod spritebatch;
mod spritesheet;

use glium::{glutin, Surface};
use glium::index::PrimitiveType;
use cgmath::SquareMatrix;
use controller::Controller;
use spritebatch::SpriteBatch;
use spritesheet::SpriteSheet;

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
                    if (tex_color.z < 0.1) discard;
                    else f_color = tex_color;
                }
            "
        },
    )
        .unwrap();

    let path = "Walk.png";
    let path = std::path::Path::new(&path);
    let image = image::open(&path).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    let opengl_texture = glium::Texture2d::new(&display, image).unwrap();
    let spritesheet = SpriteSheet::new(opengl_texture, 4, 4);
    
    let draw = |controller: &Controller, x: f32, y: f32, frame : usize| {
        let mut sprite_batch = SpriteBatch::new();
    
        let x = if controller.left {
            x - 1.0f32
        } else {
            if controller.right { x + 1.0f32 } else { x }
        };
        let y = if controller.up {
            y - 1.0f32
        } else {
            if controller.down { y + 1.0f32 } else { y }
        };

        for x1 in 0 .. 200 {
            for y1 in 0 .. 200 {
                sprite_batch.add(x + ((16 * x1) as f32), y + ((16 * y1) as f32), frame / 10, &spritesheet);
            }
        }

        let frame1 = if frame < 39 {
            frame + 1
        } else {
            0
        };
    
        //let vertices = sprite_batch.quads.iter().cloned().collect::<Vec<_>>();
        let vertex_buffer = glium::VertexBuffer::new(&display, &sprite_batch.quads).unwrap();
        let index_buffer = glium::IndexBuffer::new(&display,
                                                   PrimitiveType::TrianglesList,
                                                   &sprite_batch.indices).unwrap();


        let world = cgmath::Matrix4::identity();
        let view = cgmath::Matrix4::identity();
        let projection = cgmath::ortho(0.0f32, 320.0f32, 240.0f32, 0.0f32, 0.0f32, 100.0f32);
        // building the uniforms
        let uniforms = uniform! {
            world: Into::<[[f32;4];4]>::into(world),
            view: Into::<[[f32;4];4]>::into(view),
            projection: Into::<[[f32;4];4]>::into(projection),
            tex: spritesheet.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
           
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        let draw_parameters =  glium::DrawParameters {
            /*blend: glium::Blend {
                color: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One
                },
                alpha: glium::BlendingFunction::Addition {
                    source: glium::LinearBlendingFactor::One,
                    destination: glium::LinearBlendingFactor::One
                },
                constant_value: (1.0, 1.0, 1.0, 1.0)
            },*/
            .. Default::default()
        };

        target.draw(
                  &vertex_buffer,
                  &index_buffer,
                  &program,
                  &uniforms,
                  &draw_parameters)
            .unwrap();
        target.finish().unwrap();
        
        sprite_batch.clear();
        (x, y, frame1)
    };

    let mut controller = Controller::new();
    let mut x = 0.0f32;
    let mut y = 0.0f32;
    let mut frame = 0;


    // the main loop
    let mut closed = false;
    while !closed {
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        // Break from the main loop when the window is closed.
                        glutin::WindowEvent::Closed => closed = true,
                        // Redraw the triangle when the window is resized.
                        // glutin::WindowEvent::Resized(..) => draw(rot),
                        glutin::WindowEvent::KeyboardInput { input, .. } => {
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
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        });
        let (x1, y1, frame1) = draw(&controller, x, y, frame);
        x = x1;
        y = y1;
        frame = frame1;

    }
}
