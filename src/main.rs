
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

//#![windows_subsystem = "windows"]
mod controller;
mod sprite;
mod spritebatch;
mod spritesheet;
mod animation;
mod spriterenderer;
//mod camera;

use glium::{Display,glutin, Surface};

use controller::Controller;
use spritebatch::SpriteBatch;
use spritesheet::SpriteSheet;
use animation::Animation;
use spriterenderer::SpriteRenderer;

fn draw(display:&Display, spriterenderer : &mut SpriteRenderer, spritebatch:&mut SpriteBatch, spritesheet:&SpriteSheet, animation:&mut Animation) {
    spritebatch.clear();
    animation.update(std::time::SystemTime::now());
    let x = 0.0f32;
    let y = 0.0f32;
    for x1 in 0 .. 200 {
            for y1 in 0 .. 200 {
                spritebatch.add(x + ((16 * x1) as f32), y + ((16 * y1) as f32), animation.current_frame, &spritesheet);
            }
        }

    let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        spriterenderer.draw(&mut target, spritebatch, spritesheet);
        target.finish().unwrap();
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    

    let path = "Walk.png";
    let path = std::path::Path::new(&path);
    let image = image::open(&path).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    let opengl_texture = glium::Texture2d::new(&display, image).unwrap();
    
    let spritesheet = SpriteSheet::new(opengl_texture, 4, 4);
    let mut sprite_batch = SpriteBatch::new();
    let mut animation = Animation::new(4, 64);
    let mut spriterenderer = SpriteRenderer::new(&display);
    let mut controller = Controller::new();

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

        draw(&display, &mut spriterenderer,  &mut sprite_batch, &spritesheet, &mut animation);

    }
}
