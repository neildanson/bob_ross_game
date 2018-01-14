
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
mod camera;

use glium::{Display,glutin, Surface, Texture2d};

use controller::Controller;
use spritebatch::SpriteBatch;
use spritesheet::SpriteSheet;
use animation::Animation;
use spriterenderer::SpriteRenderer;
use camera::Camera;

fn draw(display:&Display, 
    camera : &mut Camera,
    controller: &Controller,
    spriterenderer : &mut SpriteRenderer, 
    spritebatch:&mut SpriteBatch, 
    spritesheet:&SpriteSheet, 
    animation:&mut Animation,
    backgroundpritebatch : &mut SpriteBatch,
    backgroundspritesheet:&SpriteSheet, 
    backgroundanimation:&mut Animation,
    ) {
    spritebatch.clear();
    backgroundpritebatch.clear();
    animation.update(std::time::SystemTime::now());

    let mut x = camera.x;
    let mut y = camera.y;
    x = if controller.left {
        x - 1.0
    } else { x };

    x = if controller.right {
        x + 1.0
    } else { x };

    y = if controller.up {
        y - 1.0
    } else { y };

    y = if controller.down {
        y + 1.0
    } else { y };

    camera.look_at(x, y);

    //Draw background
    let x = 0.0f32;
    let y = 0.0f32;
    for x1 in 0 .. 100 {
            for y1 in 0 .. 100 {
                backgroundpritebatch.add(x + ((16 * x1) as f32), y + ((16 * y1) as f32), backgroundanimation.current_frame, backgroundspritesheet);
            }
        }

    //Draw player
    spritebatch.add(160.0f32, 120.0f32, animation.current_frame, spritesheet);



    let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        spriterenderer.draw(&mut target, backgroundpritebatch, backgroundspritesheet, camera);
        spriterenderer.draw(&mut target, spritebatch, spritesheet, camera);

        target.finish().unwrap();
}

fn load_texture(filename : &str, display : &Display) -> Texture2d {
    let path = std::path::Path::new(&filename);
    let image = image::open(&path).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    let opengl_texture = Texture2d::new(display, image).unwrap();
    opengl_texture
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let player = load_texture("Walk.png", &display);
    let background = load_texture("Background.png", &display);
    
    let playerspritesheet = SpriteSheet::new(player, 4, 4);
    let mut playeranimation = Animation::new(4, 64);
    let mut playerspritebatch = SpriteBatch::new();
    let backgroundspritesheet = SpriteSheet::new(background, 4, 4);
    let mut backgroundanimation = Animation::new(1, 100000000);
    let mut backgroundspritebatch = SpriteBatch::new();

    let mut spriterenderer = SpriteRenderer::new(&display);
    let mut controller = Controller::new();
    let mut camera = Camera::new(320.0, 240.0);

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

        draw(&display, 
            &mut camera,
            &controller,
            &mut spriterenderer,  
            &mut playerspritebatch, &playerspritesheet, &mut playeranimation,
            &mut backgroundspritebatch, &backgroundspritesheet, &mut backgroundanimation);

    }
}
