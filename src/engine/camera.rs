extern crate cgmath;
use cgmath::{Matrix4, SquareMatrix};
use engine::BoundingBox;

pub struct Camera {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    pub boundingbox: BoundingBox,
    pub world: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub ortho: Matrix4<f32>,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
        let world = Matrix4::identity();
        let view = Matrix4::identity();
        let projection = cgmath::ortho(0.0f32, width as f32, height as f32, 0.0f32, 0.0f32, 100.0f32);
        let x = 0;
        let y = 0;
        Camera {
            x: x as f32,
            y: y as f32,
            width: width as f32,
            height: height as f32,
            boundingbox: BoundingBox::new(x, y, width, height),
            world: world,
            view: view,
            ortho: projection,
        }
    }

    pub fn look_at(&mut self, x: i32, y: i32) {
        self.x = x as f32;
        self.y = y as f32;
        let x = self.x - self.width / 2.0;
        {
            let y = self.y - self.height / 2.0;
            self.boundingbox = BoundingBox::new(x as i32, y as i32, self.width as u32, self.height as u32);
        }
        let y = self.y + self.height / 2.0;

        self.ortho = cgmath::ortho(x, x + self.width, y, y - self.height, 0.0f32, 100.0f32);
    }
}
