extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate image;
extern crate rand;
extern crate rodio;

//#![windows_subsystem = "windows"]
mod constants;
mod engine;
mod player;
mod squirrel;
mod direction;

use std::time::SystemTime;
use std::rc::Rc;

use glium::{glutin, Display, Surface};
use glium::texture::SrgbTexture2d;

use engine::{Audio, Camera, Controller, SpriteBatch, SpriteRenderer, SpriteSheet};
use player::Player;
use squirrel::Squirrel;

fn draw(
    display: &Display,
    player: &mut Player,
    squirrels: &mut [Squirrel],
    camera: &mut Camera,
    controller: &Controller,
    spriterenderer: &mut SpriteRenderer,
    spritebatch: &mut SpriteBatch,
    spritesheet: Rc<SpriteSheet>,
    backgroundspritesheet: Rc<SpriteSheet>,
    squirrelspritesheet: Rc<SpriteSheet>,
) {
    let update_time = SystemTime::now();
    spritebatch.clear();
    player.update(controller, update_time);
    camera.look_at(player.x, player.y);

    //Draw background
    let x = 0;
    let y = 0;
    for x1 in 0..constants::MAP_SIZE {
        for y1 in 0..constants::MAP_SIZE {
            spritebatch.add(
                x + ((16 * x1)),
                y + ((16 * y1)),
                constants::BACKGROUND_LAYER,
                0,
                backgroundspritesheet.clone(),
                camera,
            );
        }
    }

    //Draw Squirrels
    for squirrel in &mut squirrels.into_iter() {
        squirrel.update(update_time);
        spritebatch.add(
            squirrel.x,
            squirrel.y,
            constants::ENEMY_LAYER,
            squirrel.current_animation.current_frame,
            squirrelspritesheet.clone(),
            camera,
        )
    }

    //Draw player
    spritebatch.add(
        player.x,
        player.y,
        constants::PLAYER_LAYER,
        player.current_animation.current_frame,
        spritesheet,
        camera,
    );

    let mut target = display.draw();
    target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 0.0);

    spriterenderer.draw(&mut target, spritebatch, camera);

    target.finish().unwrap();
}

fn load_texture(filename: &str, display: &Display) -> SrgbTexture2d {
    let path = std::path::Path::new(&filename);
    let image = image::open(&path).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    SrgbTexture2d::new(display, image).unwrap()
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_title("Bob Ross");
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let player = load_texture("./Assets/Graphics/Dude.png", &display);
    let background = load_texture("./Assets/Graphics/Background.png", &display);
    let squirrel = load_texture("./Assets/Graphics/Squirrel.png", &display);

    let mut spritebatch = SpriteBatch::new();
    let playerspritesheet = SpriteSheet::new(player, 4, 5);
    let backgroundspritesheet = SpriteSheet::new(background, 4, 5);
    let squirrelspritesheet = SpriteSheet::new(squirrel, 4, 4);

    let playerspritesheet = Rc::new(playerspritesheet);
    let backgroundspritesheet = Rc::new(backgroundspritesheet);
    let squirrelspritesheet = Rc::new(squirrelspritesheet);

    let mut spriterenderer = SpriteRenderer::new(&display);
    let mut controller = Controller::new();
    let mut camera = Camera::new(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT);

    let mut player = Player::new();
    let mut squirrels = Vec::new();
    for _ in 0..constants::NUM_SQUIRRELS {
        squirrels.push(Squirrel::new());
    }

    let audio = Audio::new("./Assets/Audio/MainTheme.ogg");
    audio.play();

    // the main loop
    let mut closed = false;
    while !closed {
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    // Break from the main loop when the window is closed.
                    glutin::WindowEvent::Closed => closed = true,
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
        });

        let playerspritesheet = playerspritesheet.clone();
        let backgroundspritesheet = backgroundspritesheet.clone();
        let squirrelspritesheet = squirrelspritesheet.clone();

        draw(
            &display,
            &mut player,
            &mut squirrels,
            &mut camera,
            &controller,
            &mut spriterenderer,
            &mut spritebatch,
            playerspritesheet,
            backgroundspritesheet,
            squirrelspritesheet,
        );
    }
}
