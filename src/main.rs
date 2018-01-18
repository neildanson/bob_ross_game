
#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate image;

//#![windows_subsystem = "windows"]
mod engine;
mod player;

use std::time::SystemTime;

use glium::{Display,glutin, Surface};
use glium::texture::SrgbTexture2d;

use engine::{Animation, Camera, Controller, SpriteBatch, SpriteRenderer, SpriteSheet};
use player::Player;

fn draw(display:&Display,
    player:&mut Player,
    camera : &mut Camera,
    controller: &Controller,
    spriterenderer : &mut SpriteRenderer, 
    spritebatch:&mut SpriteBatch, 
    spritesheet:&SpriteSheet, 
    backgroundpritebatch : &mut SpriteBatch,
    backgroundspritesheet:&SpriteSheet, 
    ) {
    spritebatch.clear();
    backgroundpritebatch.clear();

    camera.look_at(player.x, player.y);

    player.update(controller, SystemTime::now());

    //Draw background
    let x = 0.0f32;
    let y = 0.0f32;
    for x1 in 0 .. 500 {
            for y1 in 0 .. 500 {
                backgroundpritebatch.add(x + ((16 * x1) as f32), y + ((16 * y1) as f32), 0, backgroundspritesheet, camera);
            }
        }

    //Draw player
    spritebatch.add(player.x, player.y, player.currect_animation.current_frame, spritesheet, camera);

    let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        spriterenderer.draw(&mut target, backgroundpritebatch, backgroundspritesheet, camera);
        spriterenderer.draw(&mut target, spritebatch, spritesheet, camera);

        target.finish().unwrap();
}

fn load_texture(filename : &str, display : &Display) -> SrgbTexture2d {
    let path = std::path::Path::new(&filename);
    let image = image::open(&path).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    let opengl_texture = SrgbTexture2d::new(display, image).unwrap();
    opengl_texture
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let player = load_texture("Dude.png", &display);
    let background = load_texture("Background.png", &display);
    
    let playerspritesheet = SpriteSheet::new(player, 4, 4);
    let playeranimation = Animation::new(0, 4, 64);
    let mut playerspritebatch = SpriteBatch::new();
    let backgroundspritesheet = SpriteSheet::new(background, 4, 4);
    let mut backgroundspritebatch = SpriteBatch::new();

    let mut spriterenderer = SpriteRenderer::new(&display);
    let mut controller = Controller::new();
    let mut camera = Camera::new(320.0, 240.0);

    let mut player = Player::new(playeranimation);

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
            &mut player,
            &mut camera,
            &controller,
            &mut spriterenderer,  
            &mut playerspritebatch, &playerspritesheet,
            &mut backgroundspritebatch, &backgroundspritesheet);

    }
}
